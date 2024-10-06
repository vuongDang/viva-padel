//! Easy to use structures for our application
use crate::server_structs::DayPlanningResponse;
// use crate::tauri_invokes::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Time at which starts a booking slot
pub type StartTime = String;
/// Keys for a calendar which are days of the year 
pub type DayKey = String;

/// A filter used to specify which padel courts we want to book
pub struct Filter {
    days_of_the_week: Vec<chrono::Weekday>,
    start_time_slots: Vec<(String, String)>,
    with_outdoor: bool
}

/// The planning of courts availaibility for a day
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DayPlanning {
    pub day: String,
    pub slots: BTreeMap<StartTime, Slot>,
}

/// The available courts for a slot
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Slot {
    pub available_courts: Vec<PadelCourt>,
    pub duration: BookingDuration,
}

/// The duration of a booking
/// Note: Currently we only take care of 1h30 booking duration
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BookingDuration {
    NinetyMin(),
    TwoHours(),
    OneHour(),
}

/// Information of a padel court
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PadelCourt {
    pub name: String,
    pub is_indoor: bool,
}

use BookingDuration::*;
// Conversion to number of seconds
impl From<&BookingDuration> for usize {
    fn from(booking: &BookingDuration) -> usize {
        match booking {
            NinetyMin() => 5400,
            TwoHours() => 7200,
            OneHour() => 3600,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Calendar {
    pub days: BTreeMap<DayKey, DayPlanning>,
}

impl Calendar {
    pub fn new() -> Self {
        Calendar {
            days: BTreeMap::new(),
        }
    }

    /// We only retrieve missing days
    pub async fn retrieve(&self, days: Vec<String>) -> Self {
        let mut calendar = Calendar::new();
        for day in days.into_iter() {
            if !self.days.contains_key(&day){
                let day_planning = DayPlanning::retrieve(day.clone()).await;
                calendar.days.insert(day, day_planning);
            }
        }
        calendar
    }

    pub fn merge(&mut self, calendar: Calendar) {
        for (day, day_planning) in calendar.days.into_iter() {
            self.days.insert(day, day_planning);
        }
    }

}

impl DayPlanning {
    pub fn has_slots(&self) -> bool {
        self.slots.iter().any(|(_, slot)| !slot.available_courts.is_empty())
    }

    pub fn testcase() -> DayPlanning {
        let response = crate::testcases::day_planning_testcase::DAY_PLANNING_CASE;
        let parsed = serde_json::from_str::<DayPlanningResponse>(response);
        parsed.unwrap().into()
    }
}

impl From<DayPlanningResponse> for DayPlanning {
    fn from(server_res: DayPlanningResponse) -> DayPlanning {
        let mut slots: BTreeMap<String, Slot> = BTreeMap::new();
        let courts = server_res.courts();

        for response_court in courts.iter() {
            for response_slot in response_court.slots().iter() {
                let prices = response_slot.prices();
                // We are only interested in 1:30 bookings
                let price = prices
                    .iter()
                    .find(|price| price.duration() == usize::from(&NinetyMin()));

                if price.is_some() && price.unwrap().bookable() {
                    let court = PadelCourt {
                        name: response_court.name().into(),
                        is_indoor: response_court.is_indoor(),
                    };
                    match slots.get_mut(response_slot.start_at()) {
                        // Init the value at this time slot if key did not exist before
                        None => {
                            slots.insert(
                                response_slot.start_at().into(),
                                Slot {
                                    available_courts: vec![court],
                                    duration: NinetyMin(),
                                },
                            );
                        }
                        Some(slot) => slot.available_courts.push(court),
                    }
                }
            }
        }

        DayPlanning {
            day: server_res.day().to_string(),
            slots,
        }
    }
}
