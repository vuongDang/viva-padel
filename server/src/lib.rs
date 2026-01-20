pub mod api;
pub mod auth;
pub mod models;

use chrono::Timelike;
use serde::{Deserialize, Serialize};
use shared::models::DayPlanningResponse;
use sqlx::sqlite::SqlitePool;
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::time::sleep;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Calendar {
    pub timestamp: i64,
    pub availabilities: BTreeMap<String, DayPlanningResponse>,
}

#[derive(Clone)]
pub struct AppState {
    /// A calendar containing LeGarden availabilities
    pub calendar: Arc<RwLock<Calendar>>,
    /// SQLite database pool
    pub db: SqlitePool,
    /// Secret key for JWT signing/verification
    pub jwt_secret: String,
}

pub async fn poll_calendar(state: AppState) {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY_SECS: u64 = 5;
    const START_POLLING_TIME: u32 = 7;
    const END_POLLING_TIME: u32 = 23;
    loop {
        let time_now = chrono::Local::now().hour();
        if (START_POLLING_TIME..END_POLLING_TIME).contains(&time_now) {
            tracing::info!("Polling calendar");
            for attempt in 1..=MAX_RETRIES {
                match shared::pull_data_from_garden::get_calendar().await {
                    Ok(availabilities) => {
                        let timestamp = chrono::Local::now().timestamp();
                        let mut old_cal = state.calendar.write().expect("Failed to get mut guard");
                        old_cal.availabilities = availabilities;
                        old_cal.timestamp = timestamp;
                        tracing::info!("Calendar fetched!");
                        break;
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
                            // For now just continue
                            break;
                        }
                    }
                }
            }
        } else {
            tracing::info!("Not the time to poll, let's sleep... zzzZZZzzZZZ");
        }
        let sleep_duration = if cfg!(feature = "local_dev") {
            // Use a short interval for local development
            Duration::from_secs(10)
        } else {
            // Use a longer interval for production
            Duration::from_secs(30 * 60)
        };
        tracing::info!("Waiting for {:?} before next poll.", sleep_duration);
        sleep(sleep_duration).await;
    }
}
