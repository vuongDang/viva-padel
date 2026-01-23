use std::time::Duration;

use serde_json::json;
use viva_padel_server::Calendar;
use viva_padel_server::mock::*;
use viva_padel_server::services::legarden::NB_DAYS_IN_BATCH;

#[tokio::test]
async fn test_health_check() {
    let (server, _) = default_test_server().await;
    let response = server.get("/viva-padel/health").await;
    response.assert_status_ok();
    assert_eq!(response.text(), "OK");
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
async fn test_get_user() {
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

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
