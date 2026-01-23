use std::collections::BTreeMap;

use async_trait::async_trait;

use crate::models::legarden::*;
use crate::services::legarden::*;

pub struct LocalGardenServer;

#[async_trait]
impl LeGardenService for LocalGardenServer {
    async fn get_calendar(&self) -> Result<Availibilities, LeGardenError> {
        Ok(json_to_calendar(NB_DAYS_IN_BATCH))
    }
}

pub fn json_to_calendar(nb_days: u64) -> Availibilities {
    let today = chrono::Local::now().date_naive();
    let next_3_months = batch_dates(today, nb_days);
    let mut calendar = BTreeMap::new();
    let json_planning = testcases::legarden::json_planning_for_29_days();
    dbg!(&json_planning.len());
    dbg!(&next_3_months.len());
    for (i, date) in next_3_months.iter().enumerate() {
        let date_str = date.format(DATE_FORMAT).to_string();
        let day_planning: DayPlanningResponse =
            serde_json::from_str(json_planning.get(i % json_planning.len()).unwrap()).unwrap();
        calendar.insert(date_str, day_planning);
    }
    calendar
}
