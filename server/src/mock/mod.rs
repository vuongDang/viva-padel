#![allow(dead_code)]
pub use crate::mock::database::*;
pub use crate::mock::legarden::*;
pub use crate::mock::notifications::*;
use crate::{AppState, Calendar};
use axum_test::TestServer;
use std::sync::{Arc, RwLock};
mod database;
mod legarden;
mod notifications;

pub const JWT_SECRET_KEY: &str = "secret_key_for_testing";

pub async fn setup_test_server() -> (TestServer, AppState) {
    let db = Arc::new(InMemoryDB::new().await.unwrap());
    let legarden = Arc::new(LocalGardenServer);
    let notifications = Arc::new(TestNotificationsService);

    let state = AppState {
        calendar: Arc::new(RwLock::new(Calendar::default())),
        db: db.clone(),
        legarden,
        notifications,
        jwt_secret: JWT_SECRET_KEY.to_string(),
    };

    crate::poll_calendar(state.clone(), Some(1)).await;

    let app = crate::api::create_router(state.clone());
    let server = TestServer::new(app.into_make_service()).expect("Failed to create test server");
    (server, state)
}
