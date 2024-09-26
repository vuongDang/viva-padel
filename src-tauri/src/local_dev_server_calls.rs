//! Local version of Tauri commands with default values
//! This is for local testing

use shared::errors::Error;
use shared::app_structs::*;
use std::time::Duration;
use async_std::task;

#[tauri::command]
pub(crate) async fn get_day_planning(day: String) -> Result<DayPlanning, Error> {
    let mut day_planning = DayPlanning::testcase();
    day_planning.day = day;
    task::sleep(Duration::from_millis(1000)).await;
    Ok(day_planning)
}

