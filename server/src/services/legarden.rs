use std::collections::BTreeMap;

use crate::models::legarden::{Availibilities, DayPlanningResponse};
use async_trait::async_trait;
use chrono::NaiveDate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeGardenError {
    #[error("Error connecting to LeGarden server: {0}")]
    ConnectionIssue(#[from] reqwest::Error),
    #[error("Parsing error: {0}")]
    Parsing(#[from] serde_json::Error),
}

/// Day format that the server uses
pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";

pub const OPENING_TIME: &str = "09:00";
pub const CLOSING_TIME: &str = "22:00";

pub const DAYS_PER_WEEK: u8 = 7;
pub const NB_WEEKS_SHOWN: u8 = 4;
pub const NB_DAYS_PER_BATCH: u8 = DAYS_PER_WEEK * NB_WEEKS_SHOWN;
pub const NB_DAYS_IN_BATCH: u64 = 90;

const BASE_URL: &str = "https://api-v3.doinsport.club/clubs/playgrounds";
const CLUB_ID: &str = "club.id=a126b4d4-a2ee-4f30-bee3-6596368368fb";
const PADEL_ID: &str = "activities.id=ce8c306e-224a-4f24-aa9d-6500580924dc";
const OPENING_HOUR: &str = "08:00";
const CLOSING_HOUR: &str = "23:00";

#[async_trait]
pub trait LeGardenService: Send + Sync {
    async fn get_calendar(&self) -> Result<Availibilities, LeGardenError>;
}

pub struct LeGardenServer;

#[async_trait]
impl LeGardenService for LeGardenServer {
    async fn get_calendar(&self) -> Result<Availibilities, LeGardenError> {
        let today = chrono::Local::now().date_naive();
        let next_3_months = batch_dates(today, NB_DAYS_IN_BATCH);
        let mut calendar = BTreeMap::new();
        for date in next_3_months {
            let date_str = date.format(DATE_FORMAT).to_string();
            let day_planning = get_day_planning(&date_str).await?;
            calendar.insert(date_str, day_planning);
        }
        Ok(calendar)
    }
}

fn day_planning_url(date: &str) -> String {
    format!(
        "{BASE_URL}/plannings/{date}?{CLUB_ID}&from={OPENING_HOUR}&to={CLOSING_HOUR}&{PADEL_ID}&bookingType=Unique",
    )
}

/// Pull availability data for LeGarden availability
pub async fn get_day_planning(date: &str) -> Result<DayPlanningResponse, LeGardenError> {
    let request = day_planning_url(date);
    let req_result: String = reqwest::get(request)
        .await
        .map_err(|e| LeGardenError::ConnectionIssue(e.into()))?
        .text()
        .await?;
    let parsed = serde_json::from_str::<DayPlanningResponse>(&req_result)
        .map_err(|e| LeGardenError::Parsing(e.into()))?;
    Ok(parsed)
}

// Create a vector of following next `nb_days` dates
pub(crate) fn batch_dates(from: NaiveDate, nb_days: u64) -> Vec<NaiveDate> {
    (0..nb_days)
        .map(|i| {
            from.checked_add_days(chrono::Days::new(i as u64))
                .expect("Failed to add days")
        })
        .collect()
}
