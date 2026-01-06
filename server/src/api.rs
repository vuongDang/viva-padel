use crate::{AppState, Calendar};
use axum::{Json, extract::State};
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/calendar", get(get_calendar))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

pub(crate) async fn get_calendar(State(state): State<AppState>) -> Json<Calendar> {
    let cal = state.calendar.read().expect("Failed to read calendar");
    Json(cal.clone())
}

pub(crate) async fn health_check() -> &'static str {
    "OK"
}
