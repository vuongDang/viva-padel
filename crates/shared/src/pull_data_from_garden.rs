use chrono::NaiveDate;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::models::*;
use crate::{models, DATE_FORMAT};

const BASE_URL: &str = "https://api-v3.doinsport.club/clubs/playgrounds";
const CLUB_ID: &str = "club.id=a126b4d4-a2ee-4f30-bee3-6596368368fb";
const PADEL_ID: &str = "activities.id=ce8c306e-224a-4f24-aa9d-6500580924dc";
const OPENING_HOUR: &str = "08:00";
const CLOSING_HOUR: &str = "23:00";

fn day_planning_url(date: &str) -> String {
    format!(
            "{BASE_URL}/plannings/{date}?{CLUB_ID}&from={OPENING_HOUR}&to={CLOSING_HOUR}&{PADEL_ID}&bookingType=Unique",
        )
}

/// Pull availability data for LeGarden availability
pub async fn get_day_planning(date: &str) -> Result<DayPlanningResponse, Error> {
    let request = day_planning_url(date);
    let req_result: String = reqwest::get(request).await?.text().await?;
    let parsed = serde_json::from_str::<models::DayPlanningResponse>(&req_result)?;
    Ok(parsed)
}

const NB_DAYS_IN_BATCH: u64 = 90;

#[cfg(feature = "local_dev")]
pub async fn get_calendar() -> Result<BTreeMap<String, DayPlanningResponse>, Error> {
    let today = chrono::Local::now().date_naive();
    let next_3_months = batch_dates(today, NB_DAYS_IN_BATCH);
    let mut calendar = BTreeMap::new();
    let json_planning = testcases::legarden::json_planning_for_29_days();
    for (i, date) in next_3_months.iter().enumerate() {
        let date_str = date.format(DATE_FORMAT).to_string();
        let day_planning: DayPlanningResponse =
            serde_json::from_str(json_planning.get(i % json_planning.len()).unwrap()).unwrap();
        calendar.insert(date_str, day_planning);
    }
    Ok(calendar)
}

#[cfg(not(feature = "local_dev"))]
pub async fn get_calendar() -> Result<BTreeMap<String, DayPlanningResponse>, Error> {
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

fn batch_dates(from: NaiveDate, nb_days: u64) -> Vec<NaiveDate> {
    (0..nb_days)
        .map(|i| {
            from.checked_add_days(chrono::Days::new(i as u64))
                .expect("Failed to add days")
        })
        .collect()
}
