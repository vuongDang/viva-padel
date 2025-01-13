//! Easy to use structures for our application
use shared::{server_structs::DayPlanningResponse, DATE_FORMAT, utils::*, NB_DAYS_PER_BATCH, filter::Filter};
use chrono::Datelike;
use leptos::{SignalGet, create_resource, Signal, SignalGetUntracked};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::cmp::{PartialEq, Eq};
use tracing::*;

/// Time at which starts a booking slot
pub type StartTime = String;
/// Keys for a calendar which are days of the year 
pub type DateKey = String;

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

/// A calendar on which a filter was applied
#[derive(Debug, Clone, Default)]
pub struct FilteredCalendar {
     pub calendar: BTreeMap<String, Signal<Option<DayPlanning>>>,
}

/// The different state that can be observed when loading a day from the calendar
#[derive(Debug, PartialEq, Eq)]
pub enum CalendarDayState {
    /// The date was not loaded on the calendar
    NotLoaded(),
    /// The date is being loaded
    Loading(),
    /// The date has been loaded but was not matching the filter criteria
    NoAvailaibility(),
    /// The date has been loaded and match the filter criteria
    Loaded(DayPlanning)
}

impl FilteredCalendar {
    /// Produce a trimmed calendar based on the provided filter
    /// If filter is None then use default filter that allows everything
    pub fn new(calendar: Calendar, filter: Option<Filter>) -> Self {
        trace!("Creating a filtered calender, this should only be triggered when:\n\t- Calendar is loading new dates\n\t- Active filter is changed ");

        let filter = filter.unwrap_or_default();
        let filtered_calendar = 
        calendar.days.into_iter().map(|(date, dp_resource)| {
            let filter_clone = filter.clone();
            (date, 
                Signal::derive(move || { 
                    trace!("Derive a new filtered day planning, this should only be triggered when the DayPlanning resource has finished loading");
                    dp_resource.get().map(|dp| DayPlanning::filtered(&dp, &filter_clone))
                })
            )
        }).collect();

        FilteredCalendar { calendar: filtered_calendar }
    }

    /// Retrieve a `DayPlanning` with information on its current state:
    /// Not loaded, loading, no availaibility, loaded
    pub fn get(&self, date: &str) -> CalendarDayState {
        match self.calendar.get(date) {
            None => CalendarDayState::NotLoaded(),
            Some( day_planning_signal) => {
                match day_planning_signal.get() {
                    None => CalendarDayState::Loading(),
                    Some(day_planning) => {
                        match day_planning.slots.is_empty() {
                            true => CalendarDayState::NoAvailaibility(),
                            false => CalendarDayState::Loaded(day_planning)
                        }
                    }
                }
            }
        }
    }

    /// Retrieve a `DayPlanning` with information on its current state as `CalendarDayState`:
    /// Not loaded, loading, no availaibility, loaded
    /// Untracked version
    #[allow(dead_code)]
    pub fn get_untracked(&self, date: &str) -> CalendarDayState {
        match self.calendar.get(date) {
            None => CalendarDayState::NotLoaded(),
            Some( day_planning_signal) => {
                match day_planning_signal.get_untracked() {
                    None => CalendarDayState::Loading(),
                    Some(day_planning) => {
                        match day_planning.slots.is_empty() {
                            true => CalendarDayState::NoAvailaibility(),
                            false => CalendarDayState::Loaded(day_planning)
                        }
                    }
                }
            }
        }
    }
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
    

    /// Load the plannings for the next batch of days 
    pub fn load_batch(&mut self) {
        debug!("Retrieving new batch");
        let first_day_to_load = if self.days.is_empty() {
            // If not initialized first day to load is today
            chrono::Local::now().date_naive()
        } else {
            // If initialized first day to load is last day + 1
            let last_date_loaded = self.days.keys().last().unwrap();
            chrono::NaiveDate::parse_from_str(last_date_loaded, DATE_FORMAT)
                .expect("Failed to parse date").checked_add_days(chrono::Days::new(1 )).expect("Failed to find last_date_loaded")
        };

        // The batch of dates we want to load
        let dates: Vec<String> =  (0..NB_DAYS_PER_BATCH)
                    .map(|i| {
                        let next_day = first_day_to_load 
                            .checked_add_days(chrono::Days::new(i as u64))
                            .unwrap();
                        next_day.format(DATE_FORMAT).to_string()
                    })
                    .collect();

        for date in dates.into_iter() {
            self.days.entry(date.clone()).or_insert(create_resource(
                || (),
                move |_| {
                    let date_clone = date.clone();
                    async move { 
                        let res = DayPlanning::retrieve(&date_clone).await.expect("Failed to retrieve new days to update calendar") ;
                        trace!("Finished retrieving {:?}", date_clone);
                        res
                    }
                }
            ));
        }
    }


    #[allow(dead_code)]
    pub fn testcase_no_ressource() -> BTreeMap<String, DayPlanning> {
        let plannings = testcases::json_planning_for_29_days();
        let dates = flatten_days(get_next_days_from(chrono::Local::now()));
        std::iter::zip(plannings, dates).map(|(planning, date) | {
            let day_planning: DayPlanning = serde_json::from_str::<DayPlanningResponse>(&planning).unwrap().into();
            if day_planning.slots.is_empty() && date == "2024-11-20" {
                println!("{:?} ---- {:?}", date, day_planning);
                println!("{:?}", planning);
            }
            (date, day_planning)
        }).collect()
    }


    #[allow(dead_code)]
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

}


impl std::fmt::Debug for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let days_debug: Vec<(String, Option<DayPlanning>)> = self.days.iter().map(|(day, resource)| (day.to_string(), resource.get())).collect();
        f.debug_struct("Calendar")
            .field("days", &days_debug)
            .finish()

    }
}


impl DayPlanning {

    pub fn new(day: &str, slots: BTreeMap<String, Slot>) -> Self {
        DayPlanning {
            weekday: day.into(),
            slots
        }
    }

    #[allow(dead_code)]
    pub fn has_slots(&self) -> bool {
        self.slots.iter().any(|(_, slot)| !slot.available_courts.is_empty())
    }

    #[allow(dead_code)]
    pub fn testcase() -> DayPlanning {
        let response = testcases::json_planning_for_1_day();
        let parsed = serde_json::from_str::<DayPlanningResponse>(&response);
        parsed.unwrap().into()
    }

    pub fn filtered(&self, filter: &Filter) -> Self {
        // Check if day match the filter
        if !filter.days_of_the_week.contains(&self.weekday) {
            return DayPlanning::new(&self.weekday, BTreeMap::new());
        }

        // Check time slots 
        let slots_filtered: BTreeMap<StartTime, Slot> =
        self.slots.clone().into_iter().filter_map(|(start_time, mut slot)| {
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
            } 

            if slot.available_courts.is_empty() {
                None
            } else {
                Some((start_time.clone(), slot.clone()))
            }
        }).collect();
        DayPlanning::new(&self.weekday, slots_filtered)
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
        let filtered = FilteredCalendar::new(cal, Some(lunch_filter));

        assert_eq!(filtered.get("2024-11-12"), CalendarDayState::NotLoaded());
        assert_eq!(filtered.get("2024-11-13"), CalendarDayState::NoAvailaibility());
        assert_eq!(filtered.get("2024-11-14"), CalendarDayState::NoAvailaibility());
        assert_eq!(filtered.get("2024-11-15"), CalendarDayState::NoAvailaibility());
        let day_planning = filtered.get("2024-11-16");
        match day_planning {
            CalendarDayState::Loaded(dp) => {
                assert_eq!(dp.slots.get("12:15").unwrap().available_courts.len(), 1);

            }
            _ => panic!("Wrong result")
        }
        assert_eq!(filtered.get("2024-11-19"), CalendarDayState::NoAvailaibility());
        assert_eq!(filtered.get("2024-11-20"), CalendarDayState::NotLoaded());

    }

    #[test]
    fn test_filtered_calendar_testcase() {
        let lunch_filter = Filter { name: "toto".into(), days_of_the_week: vec!["Mon".into(), "Tue".into(), "Wed".into(), "Thu".into(), "Fri".into()], start_time_slots: vec![("11:30".into(), "14:00".into())], with_outdoor: false};
        let mut cal = Calendar::new();
        cal.days.insert("2024-11-19".into(), new_day_planning_resource(DayPlanning::testcase()));
        let filtered = FilteredCalendar::new(cal, Some(lunch_filter));
        assert_eq!(filtered.get("2024-11-12"), CalendarDayState::NotLoaded());
        let day_planning = filtered.get("2024-11-19");
        match day_planning {
            CalendarDayState::Loaded(dp) => {
                assert!(!dp.slots.get("13:00").unwrap().available_courts.is_empty());
                assert!(!dp.slots.get("14:00").unwrap().available_courts.is_empty());
            }
            _ => panic!("Wrong result")
        }

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

    fn new_day_planning_resource(day: DayPlanning) -> leptos::Resource<(), DayPlanning> {
        create_resource(move || (), move |_| { 
            let day_clone = day.clone();
            async move { day_clone }
        })
    }

}
