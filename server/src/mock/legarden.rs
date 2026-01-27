use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

use async_trait::async_trait;
use std::sync::atomic::Ordering;

use crate::models::legarden::*;
use crate::services::legarden::*;

// A mock implementation of LeGardenService
// Returns a loop of provided availabilities
pub struct MockLeGardenService {
    availabilities: Arc<RwLock<Vec<Availabilities>>>,
    index: AtomicUsize,
    poll_interval: Duration,
}

#[async_trait]
impl LeGardenService for MockLeGardenService {
    async fn get_calendar(&self) -> Result<Availabilities, LeGardenError> {
        let data_len = self.availabilities.read().unwrap().len();
        let cal = self
            .availabilities
            .read()
            .unwrap()
            .get(self.index.fetch_add(1, Ordering::SeqCst) % data_len)
            .unwrap()
            .clone();
        Ok(cal)
    }

    // Accept all hours
    fn polling_time(&self) -> Range<u32> {
        0..25
    }

    // Poll every 30 minutes
    fn polling_interval(&self) -> Duration {
        self.poll_interval
    }
}

impl MockLeGardenService {
    pub fn new(availabilities: Vec<Availabilities>, poll_interval: Duration) -> Self {
        MockLeGardenService {
            availabilities: Arc::new(RwLock::new(availabilities)),
            index: AtomicUsize::new(0),
            poll_interval,
        }
    }
}

impl Default for MockLeGardenService {
    fn default() -> Self {
        MockLeGardenService::new(
            vec![real_data_availabilities(NB_DAYS_IN_BATCH)],
            Duration::from_secs(15),
        )
    }
}

pub fn real_data_availabilities(nb_days: u64) -> Availabilities {
    let today = chrono::Local::now().date_naive();
    let next_3_months = batch_dates(today, nb_days);
    let mut calendar = BTreeMap::new();
    let json_planning = testcases::legarden::json_planning_for_29_days();
    for (i, date) in next_3_months.iter().enumerate() {
        let date_str = date.format(DATE_FORMAT).to_string();
        let day_planning: DayPlanningResponse =
            serde_json::from_str(json_planning.get(i % json_planning.len()).unwrap()).unwrap();
        calendar.insert(date_str, day_planning);
    }
    Availabilities(calendar)
}

pub fn simple_availabilities(nb_days: u64, json: String) -> Availabilities {
    let today = chrono::Local::now().date_naive();
    let next_days = batch_dates(today, nb_days);
    let mut calendar = BTreeMap::new();
    let day_planning: DayPlanningResponse = serde_json::from_str(&json).unwrap();
    for date in next_days.iter() {
        let date_str = date.format(DATE_FORMAT).to_string();
        calendar.insert(date_str, day_planning.clone());
    }
    Availabilities(calendar)
}
