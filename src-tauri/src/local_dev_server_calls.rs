//! Local version of Tauri commands with default values
//! This is for local testing

use async_std::task;
use shared::errors::Error;
use shared::{server_structs::DayPlanningResponse, utils::*};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::Duration;

static CALENDAR: OnceLock<BTreeMap<String, DayPlanningResponse>> = OnceLock::new();

#[tauri::command]
pub(crate) async fn get_date_planning(date: String) -> Result<DayPlanningResponse, Error> {
    let calendar = CALENDAR.get_or_init(init_local_calendar);
    let day_planning = calendar.get(&date).cloned().unwrap_or_default();
    task::sleep(Duration::from_millis(1000)).await;
    Ok(day_planning)
}

pub fn init_local_calendar() -> BTreeMap<String, DayPlanningResponse> {
    let plannings = testcases::json_planning_for_29_days();
    let dates = flatten_days(get_next_days_from(chrono::Local::now()));
    std::iter::zip(plannings, dates)
        .map(|(planning, date)| {
            let day_planning: DayPlanningResponse =
                serde_json::from_str::<DayPlanningResponse>(&planning).unwrap();
            (date, day_planning)
        })
        .collect()
}
