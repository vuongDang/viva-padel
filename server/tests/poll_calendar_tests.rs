use chrono::NaiveTime;
use mockito::Server;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use viva_padel_server::{
    AppState, Calendar,
    models::{Alarm, CourtType},
    poll_calendar,
};

#[tokio::test]
async fn test_poll_calendar_lifecycle() {
    // 1. Setup mock server for Expo Push
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/--/api/v2/push/send")
        .with_status(200)
        .with_body(r#"{"data": {"status": "ok"}}"#)
        .create_async()
        .await;

    // 2. Setup database and state
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let state = AppState {
        calendar: Arc::new(RwLock::new(Calendar::default())),
        db: pool.clone(),
        jwt_secret: "secret".to_string(),
    };

    // 3. Setup test user and device
    let user_id = Uuid::new_v4();
    let ts = chrono::Utc::now().timestamp();
    sqlx::query!(
        "INSERT INTO users (id, email, created_at) VALUES (?, ?, ?)",
        user_id,
        "poll-test@example.com",
        ts
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO devices (device_id, user_id, last_seen, notif_token) VALUES (?, ?, ?, ?)",
        "device-id",
        user_id,
        ts,
        "notif-token"
    )
    .execute(&pool)
    .await
    .unwrap();

    // Active alarm for the test
    let alarm = Alarm {
        name: "Test Alarm".to_string(),
        is_active: true,
        court_type: CourtType::Both,
        time_range: (
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        ),
        ..Default::default()
    };
    let alarm_json = serde_json::to_string(&alarm).unwrap();

    let id = Uuid::new_v4();
    let ts = chrono::Utc::now().timestamp();
    sqlx::query!(
        "INSERT INTO alarms (id, user_id, alarm_json, is_active, created_at) VALUES (?, ?, ?, ?, ?)",
        id,
        user_id,
        alarm_json,
        true,
        ts
    )
    .execute(&pool)
    .await
    .unwrap();

    // 4. Run poll_calendar once
    // Note: Since shared::pull_data_from_garden::get_calendar() provides mock data in local_dev,
    // and notify_users_for_freed_courts is called inside a tokio::spawn,
    // we just need to verify that state.calendar is eventually populated.

    poll_calendar(state.clone(), Some(1)).await;

    // 5. Verify results
    let cal = state.calendar.read().unwrap();
    assert!(
        !cal.availabilities.is_empty(),
        "Calendar should be populated"
    );
    assert!(cal.timestamp > 0);
}
