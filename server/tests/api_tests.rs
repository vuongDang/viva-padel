use std::time::Duration;

use serde_json::json;
use uuid::Uuid;
use viva_padel_server::Calendar;
use viva_padel_server::mock::*;
use viva_padel_server::models::{Device, User};
use viva_padel_server::services::legarden::NB_DAYS_IN_BATCH;

#[tokio::test]
async fn test_health_check() {
    let (server, _) = default_test_server().await;
    let response = server.get("/viva-padel/health").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn test_signup_and_login() {
    let (server, state) = default_test_server().await;
    let email = "test@example.com";

    // 1. Signup
    let signup_response = server
        .post("/viva-padel/signup")
        .json(&json!({ "email": email }))
        .await;

    signup_response.assert_status_success(); // 201 Created
    let user_json: serde_json::Value = signup_response.json();
    assert_eq!(user_json["email"], email);

    // Verify in database
    let db_user = sqlx::query!("SELECT email FROM users WHERE email = ?", email)
        .fetch_one(state.db.get_db_pool())
        .await
        .expect("User should exist in database");
    assert_eq!(db_user.email, email);

    // 2. Login
    let login_response = server
        .post("/viva-padel/login")
        .json(&json!({ "email": email }))
        .await;

    login_response.assert_status_ok();
    let login_data: serde_json::Value = login_response.json();
    let token = login_data["token"]
        .as_str()
        .expect("token should be a string");

    // 3. Verify JWT token
    use jsonwebtoken::{DecodingKey, Validation, decode};
    use viva_padel_server::{auth::Claims, models::User};

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
        &Validation::default(),
    )
    .expect("Failed to decode token");

    // Verify user id in token matches the created user
    let user: User = serde_json::from_value(user_json).expect("Failed to deserialize User");
    assert_eq!(token_data.claims.sub, user.id.to_string());
}

#[tokio::test]
async fn test_get_calendar() {
    let (server, _) = default_test_server().await;
    tokio::time::sleep(Duration::from_millis(500)).await;
    let response = server.get("/viva-padel/calendar").await;
    response.assert_status_ok();

    let cal: Calendar = serde_json::from_value(response.json()).unwrap();
    assert_eq!(cal.availabilities.len(), NB_DAYS_IN_BATCH as usize);
}

#[tokio::test]
async fn test_get_user_and_notif() {
    let (server, state) = default_test_server().await;
    let email = "user@example.com";

    // Signup
    server
        .post("/viva-padel/signup")
        .json(&json!({ "email": email }))
        .await;

    let login_response = server
        .post("/viva-padel/login")
        .json(&json!({ "email": email }))
        .await;
    let token = login_response.json::<serde_json::Value>()["token"]
        .as_str()
        .unwrap()
        .to_string();

    // Get user
    let user_response = server
        .get("/viva-padel/user")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token),
        )
        .await;

    user_response.assert_status_success();
    let response: viva_padel_server::api::GetUserResponse = user_response.json();
    assert_eq!(response.user.email, email);
    assert!(response.alarms.is_empty());

    // 2. Add an alarm
    use viva_padel_server::models::Alarm;
    let alarm = Alarm::default();
    let alarms_response = server
        .post("/viva-padel/alarms")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token),
        )
        .json(&json!({ "alarms": [alarm] }))
        .await;
    alarms_response.assert_status_success();

    // 3. Get user again and verify alarm is there
    let user_response = server
        .get("/viva-padel/user")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token),
        )
        .await;

    user_response.assert_status_success();
    let response: viva_padel_server::api::GetUserResponse = user_response.json();
    assert_eq!(response.alarms.len(), 1);
    assert_eq!(response.alarms[0].name, alarm.name);

    // 4. Verify in database directly
    let user_id = uuid::Uuid::parse_str(&response.user.id.to_string()).unwrap();
    let db_alarm = sqlx::query!("SELECT alarm_json FROM alarms WHERE user_id = ?", user_id)
        .fetch_one(state.db.get_db_pool())
        .await
        .expect("Alarm should exist in database");

    // Verify if alarm was correctly saved
    let saved_alarm: Alarm = serde_json::from_str(&db_alarm.alarm_json).unwrap();
    assert_eq!(saved_alarm.name, alarm.name);
}

#[tokio::test]
async fn test_signup_invalid_email() {
    let (server, _) = default_test_server().await;

    let response = server
        .post("/viva-padel/signup")
        .json(&json!({ "email": "invalid-email" }))
        .await;

    response.assert_status_bad_request();
    let error: serde_json::Value = response.json();
    assert!(error["error"].as_str().unwrap().contains("email"));
}

#[tokio::test]
async fn test_register_device() {
    let (server, state) = default_test_server().await;
    let email = "test@example.com";

    // 1. Signup
    let signup_response = server
        .post("/viva-padel/signup")
        .json(&json!({ "email": email }))
        .await;

    signup_response.assert_status_success(); // 201 Created
    let user_json: serde_json::Value = signup_response.json();
    assert_eq!(user_json["email"], email);

    // Verify in database
    let db_user: User = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(state.db.get_db_pool())
        .await
        .expect("User should exist in database");
    assert_eq!(db_user.email, email);

    // 2. Login
    let login_response = server
        .post("/viva-padel/login")
        .json(&json!({ "email": email }))
        .await;

    login_response.assert_status_ok();
    let login_data: serde_json::Value = login_response.json();
    let token = login_data["token"]
        .as_str()
        .expect("token should be a string");

    // Register mobile device
    let notif_token = "notif_token";
    let device_id = Uuid::new_v4().to_string();
    let mobile_register_resp = server
        .post("/viva-padel/register-device")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token),
        )
        .json(&json!(Device::new_mobile(&device_id, notif_token)))
        .await;
    mobile_register_resp.assert_status_ok();
    // Check mobile in database
    let db_mobile = sqlx::query!(
        "SELECT device_id, notif_token FROM devices WHERE user_id = ?",
        db_user.id
    )
    .fetch_one(state.db.get_db_pool())
    .await
    .expect("User should exist in database");
    assert_eq!(db_mobile.device_id.unwrap(), device_id);
    assert_eq!(db_mobile.notif_token, notif_token);

    // Register browser device
    let browser_id = "browser";
    let sub_token = web_push::SubscriptionInfo::default();
    let browser_register_resp = server
        .post("/viva-padel/register-device")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token),
        )
        .json(&json!(Device::new_browser(browser_id, sub_token.clone())))
        .await;
    browser_register_resp.assert_status_ok();

    // Check browsers in database
    let db_browser = sqlx::query!(
        "SELECT browser_id, endpoint, p256dh, auth FROM browsers WHERE user_id = ?",
        db_user.id
    )
    .fetch_one(state.db.get_db_pool())
    .await
    .expect("User should exist in database");
    assert_eq!(&db_browser.browser_id.unwrap(), browser_id);
    assert_eq!(&db_browser.endpoint, &sub_token.endpoint);
    assert_eq!(&db_browser.p256dh, &sub_token.keys.p256dh);
    assert_eq!(&db_browser.auth, &sub_token.keys.auth);

    // Check that user info contain both devices
    let user_response = server
        .get("/viva-padel/user")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token),
        )
        .await;

    user_response.assert_status_success();
    let response: viva_padel_server::api::GetUserResponse = user_response.json();
    assert_eq!(response.devices.len(), 2);
    assert_eq!(response.devices[0].device_id, device_id);
    assert_eq!(response.devices[1].device_id, browser_id);
}
