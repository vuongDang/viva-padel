mod availaibility_calendar;
mod day_availability;
mod filter;
mod next_courts;

use crate::book_court::availaibility_calendar::AvailaibilityCalendar;
use crate::book_court::filter::FilterView;
use crate::book_court::next_courts::NextCourtsView;
use std::collections::BTreeMap;
use leptos::*;
use shared::frontend::calendar_ui::{Calendar, Filter, DayPlanning, DateKey};
use thaw::mobile::*;
use thaw::*;

#[component]
pub fn BookCourtView() -> impl IntoView {
    // let value = create_rw_signal(String::from("calendar"));
    let selected_tab = create_rw_signal(String::from("next_courts"));
    let filter = create_rw_signal(Filter::default());
    // Court availabilities for all the days loaded
    let calendar: RwSignal<Calendar> = create_rw_signal(Calendar::new());

    // The calendar obtained after applying the filter
    let filtered_calendar: Signal<BTreeMap<DateKey, DayPlanning>> = Signal::derive(move || {
        // leptos::logging::log!("Filtering!");
        calendar.get().filtered(&filter.get())
    });


    view! {
        <Tabs value=selected_tab>
            <Tab key="filter" label="Filter">
                <FilterView filter />
            </Tab>
            <Tab key="calendar" label="Calendar">
                <AvailaibilityCalendar filter calendar filtered_calendar />
            </Tab>
            <Tab key="next_courts" label="Next Courts">
                <NextCourtsView filter calendar filtered_calendar />
            </Tab>
        </Tabs>
    }
}

/// Ask the server for the plannings of the specified days
/// This does not reload already loaded days
pub(crate) fn update_calendar(calendar: RwSignal<Calendar>, dates: Vec<String>) {
    // Closure to update the calendar
    calendar.update(|cal| {
        for day_shown  in dates.into_iter() {
            cal.days.entry(day_shown.clone()).or_insert(create_resource(
                || (),
                move |_| {
                    let day_shown_clone = day_shown.clone();
                    async move { 
                        DayPlanning::retrieve(&day_shown_clone).await 
                    }
                }
            ));
        }
    });
    logging::log!("Updated calendar: {:?}",calendar.get_untracked().days.keys());
}
