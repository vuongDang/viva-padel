#![allow(dead_code)]
pub use crate::mock::database::*;
pub use crate::mock::legarden::*;
pub use crate::mock::notifications::*;
use crate::models::legarden::Availabilities;
use crate::services::LeGardenService;
use crate::{AppState, Calendar};
use axum_test::TestServer;
use std::sync::{Arc, RwLock};
use std::time::Duration;
mod database;
mod legarden;
mod notifications;

pub const JWT_SECRET_KEY: &str = "secret_key_for_testing";

pub async fn default_test_server() -> (TestServer, AppState) {
    setup_with_state(Arc::new(MockLeGardenService::default())).await
}

pub async fn test_server(
    availabilities: Vec<Availabilities>,
    poll_interval: Duration,
) -> (TestServer, AppState) {
    setup_with_state(Arc::new(MockLeGardenService::new(
        availabilities,
        poll_interval,
    )))
    .await
}

async fn setup_with_state(legarden: Arc<dyn LeGardenService>) -> (TestServer, AppState) {
    let db = Arc::new(MockDB::new().await.unwrap());
    let notifications = Arc::new(MockNotificationsService::default());

    let cal = Calendar {
        timestamp: 0,
        availabilities: legarden.get_calendar().await.unwrap(),
    };
    let state = AppState {
        calendar: Arc::new(RwLock::new(cal)),
        db,
        legarden,
        notifications,
        jwt_secret: JWT_SECRET_KEY.to_string(),
    };

    let poll_state = state.clone();
    tokio::spawn(async move {
        crate::run(poll_state).await;
    });

    let app = crate::api::create_router(state.clone());
    let server = TestServer::new(app.into_make_service()).expect("Failed to create test server");
    (server, state)
}
