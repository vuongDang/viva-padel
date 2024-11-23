//! Local version of Tauri commands with default values
//! This is for local testing

use async_std::task;
use shared::errors::Error;
use shared::frontend::calendar_ui::*;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::Duration;

static CALENDAR: OnceLock<BTreeMap<String, DayPlanning>> = OnceLock::new();

#[tauri::command]
pub(crate) async fn get_date_planning(date: String) -> Result<DayPlanning, Error> {
    let calendar = CALENDAR.get_or_init(Calendar::testcase_no_ressource);
    let day_planning = calendar
        .get(&date)
        .unwrap_or(&DayPlanning::testcase())
        .clone();
    task::sleep(Duration::from_millis(1000)).await;
    Ok(day_planning)
}
