//! Easy to use structures for our application
use crate::{server_structs::DayPlanningResponse, DATE_FORMAT, frontend::utils::*, OPENING_TIME, CLOSING_TIME};
use chrono::Datelike;
use leptos::{SignalGet, create_resource};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::cmp::{PartialEq, Eq};
use tracing::*;

/// Time at which starts a booking slot
pub type StartTime = String;
/// Keys for a calendar which are days of the year 
pub type DateKey = String;

/// A filter used to specify which padel courts we want to book
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Filter {
    pub name: String,
    pub days_of_the_week: Vec<String>,
    pub start_time_slots: Vec<(String, String)>,
    pub with_outdoor: bool
}

/// The planning of courts availaibility for a day
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct DayPlanning {
    pub weekday: String,
    pub slots: BTreeMap<StartTime, Slot>,
}

/// The available courts for a slot
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, )]
pub struct Slot {
    pub available_courts: Vec<PadelCourt>,
    pub duration: BookingDuration,
}

/// The duration of a booking
/// Note: Currently we only take care of 1h30 booking duration
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub enum BookingDuration {
    NinetyMin(),
    TwoHours(),
    OneHour(),
}

/// Information of a padel court
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
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

#[derive(Default, Clone)]
pub struct Calendar {
    pub days: BTreeMap<DateKey, leptos::Resource<(), DayPlanning>>,
}

impl Calendar {
    pub fn new() -> Self {
        Calendar {
            days: BTreeMap::new(),
        }
    }

    pub fn testcase_no_ressource() -> BTreeMap<String, DayPlanning> {
        let plannings = testcases::json_planning_for_29_days();
        let dates = flatten_days(get_next_days_from(chrono::Local::now()));
        std::iter::zip(plannings, dates).map(|(planning, date) | {
            let day_planning: DayPlanning = serde_json::from_str::<DayPlanningResponse>(&planning).unwrap().into();
            if day_planning.slots.is_empty() {
                if date == "2024-11-20" {
                    println!("{:?} ---- {:?}", date, day_planning);
                    println!("{:?}", planning);
                }
            }
            (date, day_planning)
        }).collect()
    }


    /// We only retrieve days that were not loaded before
    pub async fn retrieve(&mut self, days: Vec<String>)  {
        for day  in days.into_iter() {
            self.days.entry(day.clone()).or_insert(create_resource(|| (),
                move |_| {
                    let day_clone = day.clone();
                    async move { 
                        DayPlanning::retrieve(&day_clone).await.expect("Failed to retrieve plannings")
                    }}));
        }
    }

    /// Produce a trimmed calendar based on the provided filter
    pub fn filtered(&self, filter: &Filter) -> BTreeMap<DateKey, DayPlanning> {
        trace!("Creating a filtered calender");
        // Filter out resources that have not loaded yet
        let mut filtered_calendar: BTreeMap<DateKey, DayPlanning> = self.clone().days.into_iter().filter_map(|(day, day_planning)| day_planning.get().map(|day_planning| (day, day_planning))).collect();
        filtered_calendar = 
            filtered_calendar.into_iter().map(|(day, mut day_planning)| {
                // Check if day match the filter
                if !filter.days_of_the_week.contains(&day_planning.weekday) {
                    day_planning.slots = BTreeMap::new();
                    return (day, day_planning);
                }

                // Check time slots 
                let day_planning_filtered: BTreeMap<StartTime, Slot> =
                day_planning.slots.into_iter().filter_map(|(start_time, mut slot)| {
                    // If start time is  in any of the filter time slots
                    let slot_is_in_filter = filter.start_time_slots.iter().any(|(begin, end)| {
                        &start_time >= begin && &start_time <= end
                    });
                    if !slot_is_in_filter {
                        return None;
                    }
                    // Remove slots that only contains outdoor courts if filtered out
                    if !filter.with_outdoor {
                        slot.available_courts.retain(|court| court.is_indoor);
                        // day_planning.slots.get_mut(start_time).unwrap().available_courts.retain(|court| court.is_indoor);
                    } 

                    if slot.available_courts.is_empty() {
                        None
                    } else {

                        Some((start_time, slot))
                    }
                }).collect();

                (day.clone(), DayPlanning::new(&day_planning.weekday, day_planning_filtered))
            }).collect();
        filtered_calendar 
    }


}

impl std::fmt::Debug for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let days_debug: Vec<(String, Option<DayPlanning>)> = self.days.iter().map(|(day, resource)| (day.to_string(), resource.get())).collect();
        f.debug_struct("Calendar")
            .field("days", &days_debug)
            .finish()

    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter{
            name: "default".to_string(),
            days_of_the_week: vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].into_iter().map(|day| day.into()).collect(),
            start_time_slots: vec!((OPENING_TIME.into(), CLOSING_TIME.into())),
            with_outdoor: true
        }
    }
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool  {
        self.name == other.name
    }
}

impl Filter {
    pub fn default_filters() -> HashMap<String, Filter> {
        let mut filters = HashMap::new();
        filters.insert("default".to_string(), Filter::default());
        filters
    }
}

impl DayPlanning {

    pub fn new(day: &str, slots: BTreeMap<String, Slot>) -> Self {
        DayPlanning {
            weekday: day.into(),
            slots
        }
    }

    pub fn has_slots(&self) -> bool {
        self.slots.iter().any(|(_, slot)| !slot.available_courts.is_empty())
    }

    pub fn testcase() -> DayPlanning {
        let response = testcases::json_planning_for_1_day();
        let parsed = serde_json::from_str::<DayPlanningResponse>(&response);
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

        let weekday = chrono::NaiveDate::parse_from_str(server_res.date(), DATE_FORMAT ).expect("Failed to parse date from server day planning").weekday();

        DayPlanning {
            weekday: weekday.to_string(),
            slots,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filtered_calendar() {
        let lunch_filter = Filter { name: "toto".into(), days_of_the_week: vec!["Mon".into(), "Tue".into(), "Wed".into(), "Thu".into(), "Fri".into()], start_time_slots: vec![("11:30".into(), "14:00".into())], with_outdoor: false};
        let cal = default_calendar();
        let filtered = cal.filtered(&lunch_filter);

        // println!("{:#?}", filtered);
        assert!(filtered.get("2024-11-13").unwrap().slots.is_empty());
        assert!(filtered.get("2024-11-14").unwrap().slots.is_empty());
        assert!(filtered.get("2024-11-15").unwrap().slots.get("12:15").unwrap().available_courts.is_empty());
        assert_eq!(filtered.get("2024-11-16").unwrap().slots.get("12:15").unwrap().available_courts.len(), 1);
        assert!(filtered.get("2024-11-19").unwrap().slots.is_empty());
    }

    #[test]
    fn test_filtered_calendar_testcase() {
        let lunch_filter = Filter { name: "toto".into(), days_of_the_week: vec!["Mon".into(), "Tue".into(), "Wed".into(), "Thu".into(), "Fri".into()], start_time_slots: vec![("11:30".into(), "14:00".into())], with_outdoor: false};
        let mut cal = Calendar::new();
        cal.days.insert("2024-11-19".into(), new_day_planning_resource(DayPlanning::testcase()));
        let filtered = cal.filtered(&lunch_filter);
        println!("filtered: {:?}", filtered);
    }


    fn default_calendar() -> Calendar {
        let mut cal = Calendar::new();


        let no_courts_day = new_day_planning_resource(DayPlanning::new("Mon", BTreeMap::new()));
        let only_morning_day = new_day_planning_resource(  DayPlanning::new("Tue", morning_slots()));
        let lunch_day_only_outdoor = new_day_planning_resource( DayPlanning::new("Wed", lunch_slot_only_outdoor()));
        let lunch_day = new_day_planning_resource(DayPlanning::new("Thu", lunch_slots()));
        let weekend_day = new_day_planning_resource( DayPlanning::new("Sun", lunch_slots()));


        cal.days.insert("2024-11-13".into(), no_courts_day);
        cal.days.insert("2024-11-14".into(), only_morning_day);
        cal.days.insert("2024-11-15".into(), lunch_day_only_outdoor);
        cal.days.insert("2024-11-16".into(), lunch_day);
        cal.days.insert("2024-11-19".into(), weekend_day);
        cal
    }

    fn court9() -> PadelCourt {
        PadelCourt {
            name: "9".into(),
            is_indoor: false
        }
    }

    fn court1() -> PadelCourt {
        PadelCourt {
            name: "1".into(),
            is_indoor:true 
        }
    }

    fn only_outdoor_slot() -> Slot {
        Slot { 
            available_courts: vec![court9()], 
            duration: BookingDuration::NinetyMin()
        }
    }

    fn indoor_and_outdoor_slot() -> Slot { 
        Slot { 
            available_courts: vec![court1(), court9()], 
            duration: BookingDuration::NinetyMin()
        }
    }

    fn lunch_slot_only_outdoor() -> BTreeMap<String, Slot> { 
        let mut slots = BTreeMap::new();
        slots.insert("12:15".into(), only_outdoor_slot());
        slots
    }

    fn lunch_slots() -> BTreeMap<String, Slot> { 
        let mut slots = BTreeMap::new();
        slots.insert("12:15".into(), only_outdoor_slot());
        slots.insert("12:15".into(), indoor_and_outdoor_slot());
        slots
    }

    fn morning_slots() -> BTreeMap<String, Slot> { 
        let mut slots = BTreeMap::new();
        slots.insert("10:15".into(), indoor_and_outdoor_slot());
        slots
    }
}


