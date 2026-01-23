use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CourtType {
    Indoor,
    Outdoor,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Alarm {
    pub name: String,
    pub days_of_the_week: Vec<Weekday>,
    pub time_range: (NaiveTime, NaiveTime),
    pub court_type: CourtType,
    pub weeks_ahead: u32,
    pub is_active: bool,
    pub slot_durations: Vec<usize>,
}

impl Default for Alarm {
    fn default() -> Self {
        Self {
            name: "Default Alarm".to_string(),
            days_of_the_week: vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
                Weekday::Sat,
                Weekday::Sun,
            ],
            time_range: (
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            ),
            court_type: CourtType::Both,
            weeks_ahead: 1,
            is_active: true,
            slot_durations: vec![3600, 5400, 7200],
        }
    }
}
