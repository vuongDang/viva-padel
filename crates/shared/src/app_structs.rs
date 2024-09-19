//! Easy to use structures for our application
use crate::server_structs::DayPlanningResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type StartTime = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct DayPlanning {
    pub day: String,
    pub slots: HashMap<StartTime, Slot>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Slot {
    pub available_courts: Vec<PadelCourt>,
    pub duration: BookingDuration,
}

// Currently we only take care of 1h30 booking duration
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BookingDuration {
    NinetyMin(),
    TwoHours(),
    OneHour(),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

impl From<DayPlanningResponse> for DayPlanning {
    fn from(server_res: DayPlanningResponse) -> DayPlanning {
        let mut slots: HashMap<String, Slot> = HashMap::new();
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
