mod availaibility_calendar;
mod day_availability;
mod filter;

use crate::book_court::availaibility_calendar::AvailaibilityCalendar;
use crate::book_court::filter::FilterView;
use leptos::*;
use thaw::mobile::*;
use thaw::*;

#[component]
pub fn BookCourtView() -> impl IntoView {
    // let value = create_rw_signal(String::from("calendar"));
    let value = create_rw_signal(String::from("filter"));

    view! {
        <Tabs value>
            <Tab key="filter" label="Filter">
                <FilterView />
            </Tab>
            <Tab key="calendar" label="Calendar">
                <AvailaibilityCalendar />
            </Tab>
            <Tab key="next_courts" label="Next Courts">
                "Next courts"
            </Tab>
        </Tabs>
    }
}
