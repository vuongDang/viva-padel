mod availaibility_calendar;
mod day_availability;
mod filter;

use crate::book_court::availaibility_calendar::AvailaibilityCalendar;
use crate::book_court::filter::FilterView;
use leptos::*;
use shared::frontend::calendar_ui::Filter;
use thaw::mobile::*;
use thaw::*;

#[component]
pub fn BookCourtView() -> impl IntoView {
    // let value = create_rw_signal(String::from("calendar"));
    let selected_tab = create_rw_signal(String::from("filter"));
    let filter = create_rw_signal(Filter::default());

    view! {
        <Tabs value=selected_tab>
            <Tab key="filter" label="Filter">
                <FilterView filter />
            </Tab>
            <Tab key="calendar" label="Calendar">
                <AvailaibilityCalendar filter />
            </Tab>
            <Tab key="next_courts" label="Next Courts">
                "Next courts"
            </Tab>
        </Tabs>
    }
}
