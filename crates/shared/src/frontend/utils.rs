use crate::{DATE_FORMAT, NB_DAYS_SHOWN, DAYS_PER_WEEK, frontend::calendar_ui::DayPlanning};
use chrono::{Datelike, DateTime, Local, Weekday, Days};
use leptos::create_resource;


pub fn new_day_planning_resource(day: DayPlanning) -> leptos::Resource<(), DayPlanning> {
    create_resource(move || (), move |_| { 
        let day_clone = day.clone();
        async move { day_clone }
    })
}


/// Get `NB_DAYS_SHOWN` next days starting with the previous Monday since [`first_day`]
pub fn get_next_days_from(first_day: DateTime<Local>) -> Vec<Vec<DateTime<Local>>> {
    let now_day = first_day.weekday();
    let days_since_previous_monday = now_day.days_since(Weekday::Mon);

    // The first day we want to show is always a Monday
    let first_day_shown = first_day
        .checked_sub_days(Days::new(days_since_previous_monday as u64))
        .expect("Calendar day underflow");

    // We get all the `NB_DAYS_SHOWN` starting previous Monday
    let days_shown: Vec<DateTime<Local>> = (0..NB_DAYS_SHOWN)
        .map(|i| {
            first_day_shown
                .checked_add_days(Days::new(i as u64))
                .expect("Calendar day overflows")
        })
        .collect();

    // Split the days shown into days per week
    days_shown
        .chunks(DAYS_PER_WEEK as usize)
        .map(|s| s.into())
        .collect()
}

pub fn flatten_days(days: Vec<Vec<DateTime<Local>>>) -> Vec<String> {
    days.iter()
        .flatten()
        .map(|day_shown| day_shown.format(DATE_FORMAT).to_string())
        .collect()
}

