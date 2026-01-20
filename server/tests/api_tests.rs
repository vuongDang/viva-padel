use axum_test::TestServer;
use serde_json::json;
use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, RwLock};
use viva_padel_server::{AppState, Calendar, api::create_router};

const JWT_SECRET_KEY: &str = "secret_key_for_testing";
async fn setup_test_server() -> (TestServer, SqlitePool) {
    // Setup in-memory database for testing
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState {
        calendar: Arc::new(RwLock::new(Calendar {
            timestamp: 0,
            availabilities: shared::pull_data_from_garden::json_to_calendar(),
        })),
        db: pool.clone(),
        jwt_secret: JWT_SECRET_KEY.to_string(),
    };

    let app = create_router(state);
    let server = TestServer::new(app.into_make_service()).expect("Failed to create test server");
    (server, pool)
}

#[tokio::test]
async fn test_health_check() {
    let (server, _) = setup_test_server().await;
    let response = server.get("/viva-padel/health").await;
    response.assert_status_ok();
    assert_eq!(response.text(), "OK");
}

#[tokio::test]
async fn test_signup_and_login() {
    let (server, pool) = setup_test_server().await;
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
        .fetch_one(&pool)
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
    let (server, _) = setup_test_server().await;
    let response = server.get("/viva-padel/calendar").await;
    response.assert_status_ok();

    let cal: Calendar = serde_json::from_value(response.json()).unwrap();
    assert_eq!(cal.timestamp, 0);
    assert_eq!(cal.availabilities.len(), 90);
}

#[tokio::test]
async fn test_get_user() {
    let (server, pool) = setup_test_server().await;
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
    let db_alarm = sqlx::query!(
        "SELECT alarm_json FROM alarms WHERE user_id = ?",
        user_id
    )
    .fetch_one(&pool)
    .await
    .expect("Alarm should exist in database");

    let saved_alarm: Alarm = serde_json::from_str(&db_alarm.alarm_json).unwrap();
    assert_eq!(saved_alarm.name, alarm.name);
}
