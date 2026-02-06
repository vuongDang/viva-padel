use std::any::Any;
use std::time::Duration;

use testcases::legarden::{json_planning_simple_all_booked, json_planning_simple_day};
use tokio::time::sleep;
use viva_padel_server::mock::*;
use viva_padel_server::models::Alarm;

#[tokio::test]
async fn test_server_lifecycle() {
    let _log_guard = viva_padel_server::setup_logging();
    let avail_free = simple_availabilities(3, json_planning_simple_day());
    let avail_booked = simple_availabilities(3, json_planning_simple_all_booked());

    let (_server, state) =
        test_server(vec![avail_booked, avail_free], Duration::from_millis(500)).await;
    let email = "poll-test@example.com";
    let token = "notif_token";
    let device_id = "device";

    // Create user
    let user = state.db.create_user(email).await.unwrap();

    // Register device
    state
        .db
        .register_device(device_id, token, user.id)
        .await
        .unwrap();

    // Register alarm
    state
        .db
        .update_alarms(user.id, vec![Alarm::default()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    let notif_any: &dyn Any = state.notifications.as_ref();

    let notifications = notif_any
        .downcast_ref::<MockNotificationsService>()
        .unwrap()
        .notifications
        .lock()
        .await;

    // dbg!(&notifications);
    assert!(!notifications.is_empty())
}

#[tokio::test]
async fn test_server_no_notifications() {
    let avail_free = simple_availabilities(3, json_planning_simple_day());

    let (_server, state) = test_server(vec![avail_free], Duration::from_millis(500)).await;
    let email = "poll-test@example.com";
    let token = "notif_token";
    let device_id = "device";
    let user = state.db.create_user(email).await.unwrap();
    // Register device
    state
        .db
        .register_device(device_id, token, user.id)
        .await
        .unwrap();
    // Register alarm
    state
        .db
        .update_alarms(user.id, vec![Alarm::default()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    let notif_any: &dyn Any = state.notifications.as_ref();

    let notifications = notif_any
        .downcast_ref::<MockNotificationsService>()
        .unwrap()
        .notifications
        .lock()
        .await;

    // dbg!(&notifications);
    assert!(notifications.is_empty())
}
