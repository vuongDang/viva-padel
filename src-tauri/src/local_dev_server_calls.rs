//! Local version of Tauri commands with default values
//! This is for local testing

use async_std::task;
use shared::errors::Error;
use shared::{server_structs::DayPlanningResponse, utils::*};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::Duration;
use tauri::{path::BaseDirectory, Manager};

static CALENDAR: OnceLock<BTreeMap<String, DayPlanningResponse>> = OnceLock::new();

#[tauri::command]
pub(crate) async fn get_date_planning(
    handle: tauri::AppHandle,
    date: String,
) -> Result<DayPlanningResponse, Error> {
    let calendar = CALENDAR.get_or_init(move || init_local_calendar(handle));
    let day_planning = calendar.get(&date).cloned().unwrap_or_default();
    task::sleep(Duration::from_millis(1000)).await;
    Ok(day_planning)
}

pub fn init_local_calendar(handle: tauri::AppHandle) -> BTreeMap<String, DayPlanningResponse> {
    let plannings = json_planning_for_29_days_tauri(handle);
    let dates = flatten_days(get_next_days_from(chrono::Local::now()));
    std::iter::zip(plannings, dates)
        .map(|(planning, date)| {
            let day_planning: DayPlanningResponse =
                serde_json::from_str::<DayPlanningResponse>(&planning).unwrap();
            (date, day_planning)
        })
        .collect()
}

pub fn json_planning_for_29_days_tauri(app: tauri::AppHandle) -> Vec<String> {
    let mut calendar = vec![];
    for i in 0..=28 {
        let path = format!("resources/plannings/day({i}).json");
        let resource_path = app
            .path()
            .resolve(&path, BaseDirectory::Resource)
            .expect("Did not find planning resource");
        let content = std::fs::read_to_string(resource_path).expect("Error while getting data");
        calendar.push(content)
    }
    calendar
}
