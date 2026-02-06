use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use viva_padel_server::{AppState, Calendar, api::create_router, run, setup_logging};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // Keep the log guard in main to flush last logs if server is closed
    let _log_guard = setup_logging();
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // For production
    #[cfg(not(feature = "local_dev"))]
    let (legarden, notifications, db) = {
        let legarden = Arc::new(viva_padel_server::services::LeGardenServer);
        let notifications = Arc::new(viva_padel_server::services::ExpoNotificationsService);
        let db = Arc::new(
            viva_padel_server::services::SQLiteDB::new()
                .await
                .expect("Failed to setup database"),
        );
        (legarden, notifications, db)
    };

    // For testing
    #[cfg(feature = "local_dev")]
    let (legarden, notifications, db) = {
        use std::time::Duration;
        use testcases::legarden::{json_planning_simple_all_booked, json_planning_simple_day};
        use viva_padel_server::mock::simple_availabilities;
        let avail_free = simple_availabilities(3, json_planning_simple_day());
        let avail_booked = simple_availabilities(3, json_planning_simple_all_booked());
        let legarden = Arc::new(viva_padel_server::mock::MockLeGardenService::new(
            vec![avail_booked, avail_free],
            Duration::from_secs(10),
        ));
        // let legarden = Arc::new(viva_padel_server::mock::MockLeGardenService::default());
        let notifications = Arc::new(viva_padel_server::mock::MockNotificationsService::default());
        // let notifications = Arc::new(viva_padel_server::services::ExpoNotificationsService);
        let db = Arc::new(
            viva_padel_server::services::SQLiteDB::new()
                .await
                .expect("Failed to setup database"),
        );
        (legarden, notifications, db)
    };

    let state = AppState {
        calendar: Arc::new(RwLock::new(Calendar::default())),
        db,
        legarden,
        notifications,
        jwt_secret,
    };

    // Poll LeGarden server to get courts availabilities
    let poll_state = state.clone();
    tokio::spawn(async move {
        run(poll_state).await;
    });

    let app = create_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
