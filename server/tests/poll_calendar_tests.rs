use mockito::Server;
use viva_padel_server::mock::*;
use viva_padel_server::{models::Alarm, poll_calendar};

#[tokio::test]
async fn test_poll_calendar_lifecycle() {
    let (_server, state) = setup_test_server().await;
    let email = "poll-test@example.com";
    let token = "notif_token";
    let device_id = "device";
    let user = state.db.create_user(email).await.unwrap();
    state
        .db
        .register_device(device_id, token, user.id)
        .await
        .unwrap();
    state
        .db
        .update_alarms(user.id, vec![Alarm::default()])
        .await
        .unwrap();

    // 1. Setup mock server for Expo Push
    // let mut server = Server::new_async().await;
    // let _m = server
    //     .mock("POST", "/--/api/v2/push/send")
    //     .with_status(200)
    //     .with_body(r#"{"data": {"status": "ok"}}"#)
    //     .create_async()
    //     .await;

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
