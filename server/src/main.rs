mod api;
use api::*;

use serde::Serialize;
use shared::models::DayPlanningResponse;
use std::{
    collections::BTreeMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::time::sleep;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Calendar {
    pub timestamp: i64,
    pub availabilities: BTreeMap<String, DayPlanningResponse>,
}

#[derive(Clone)]
pub struct AppState {
    /// A calendar containing LeGarden availabilities
    pub calendar: Arc<RwLock<Calendar>>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        calendar: Arc::new(RwLock::new(Calendar::default())),
    };

    // Poll LeGarden server to get courts availabilities
    let mut poll_state = state.clone();
    tokio::spawn(async move {
        poll_calendar(&mut poll_state).await;
    });

    let app = create_router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn poll_calendar(state: &mut AppState) {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY_SECS: u64 = 5;
    loop {
        tracing::info!("Polling calendar from");
        for attempt in 1..=MAX_RETRIES {
            match shared::pull_data_from_garden::get_calendar().await {
                Ok(availabilities) => {
                    let timestamp = chrono::Local::now().timestamp();
                    let mut old_cal = state.calendar.write().expect("Failed to get mut guard");
                    old_cal.availabilities = availabilities;
                    old_cal.timestamp = timestamp;
                }
                Err(e) => {
                    if attempt < MAX_RETRIES {
                        tracing::warn!("Failed to get calendar: {e}. Retrying...");
                        sleep(Duration::from_secs(RETRY_DELAY_SECS)).await;
                    } else {
                        tracing::error!(
                            "Could not get calendar after {MAX_RETRIES} attempts. Giving up"
                        );
                        // Decide what to do if we can't fetch the calendar
                        todo!()
                    }
                }
            }
        }
        sleep(Duration::from_mins(30)).await;
    }
}
