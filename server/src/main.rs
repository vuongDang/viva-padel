use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use viva_padel_server::{AppState, Calendar, api::create_router, poll_calendar};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    #[cfg(not(feature = "local_dev"))]
    let db = Arc::new(
        viva_padel_server::services::SQLiteDB::new()
            .await
            .expect("Failed to setup database"),
    );
    #[cfg(not(feature = "local_dev"))]
    let legarden = Arc::new(viva_padel_server::services::LeGardenService);
    #[cfg(not(feature = "local_dev"))]
    let notifications = Arc::new(viva_padel_server::services::ExpoNotificationsService);

    #[cfg(feature = "local_dev")]
    let db = Arc::new(
        viva_padel_server::mock::InMemoryDB::new()
            .await
            .expect("Failed to setup database"),
    );
    #[cfg(feature = "local_dev")]
    let legarden = Arc::new(viva_padel_server::mock::LocalGardenServer);
    #[cfg(feature = "local_dev")]
    let notifications = Arc::new(viva_padel_server::mock::TestNotificationsService);

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
        poll_calendar(poll_state, None).await;
    });

    let app = create_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
