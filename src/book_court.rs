mod availaibility_calendar;
mod day_availability;
mod filter_builder;
mod next_courts;

use crate::book_court::availaibility_calendar::AvailaibilityCalendar;
use crate::book_court::filter_builder::FilterView;
use crate::book_court::next_courts::NextCourtsView;
use leptos::*;
use shared::frontend::calendar_ui::{
    Calendar, DateKey, DayPlanning, Filter, FilteredCalendar, Slot, StartTime,
};
use std::collections::HashMap;
use thaw::mobile::*;
use thaw::*;
use tracing::*;

// const DEFAULT_TAB: &str = "next_courts";
const DEFAULT_TAB: &str = "calendar";

#[component]
pub fn BookCourtView() -> impl IntoView {
    let selected_tab = create_rw_signal(String::from(DEFAULT_TAB));
    let stored_filters = Resource::once(|| async move {
        Filter::get_stored_filters()
            .await
            .expect("Failed to retrieved filters stored on disk")
    });
    let filters = create_rw_signal(None);

    // The active filter, the option type is solely to fit the selector item from Thaw UI
    let active_filter = create_rw_signal(Some(Filter::default()));

    // Court availabilities for all the days loaded
    let calendar: RwSignal<Calendar> = create_rw_signal(Calendar::new());
    // The calendar obtained after applying the filter
    let filtered_calendar: RwSignal<FilteredCalendar> =
        create_rw_signal(FilteredCalendar::default());

    let counter = create_rw_signal(0);

    provide_context(calendar);
    provide_context(filtered_calendar);
    provide_context(active_filter);

    update_calendar(calendar, filtered_calendar, active_filter, true);

    view! {
        <p>
            "Number of times calendar signal get triggered: "
            {move || {
                let _ = calendar.get();
                let _ = filtered_calendar.get();
                counter.update(|c| *c += 1);
                format!("{:?}", counter.get())
            }}
        </p>
        <Tabs value=selected_tab>
            {move || match stored_filters.get() {
                None => view! { <p>"No filters"</p> }.into_view(),
                Some(stored_filters) => {
                    filters.set(Some(stored_filters));
                    view! { <FilterSelector filters /> }
                }
            }} <Tab key="filter" label="Filter">
                <FilterView filters />
            </Tab>
            <Tab key="calendar" label="Calendar">
                <AvailaibilityCalendar />
            </Tab>
            <Tab key="next_courts" label="Next Courts">
                <NextCourtsView />
            </Tab>
        </Tabs>
    }
}

#[component]
pub(crate) fn FilterSelector(filters: RwSignal<Option<HashMap<String, Filter>>>) -> impl IntoView {
    let calendar = use_context::<RwSignal<Calendar>>().unwrap();
    let filtered_calendar = use_context::<RwSignal<FilteredCalendar>>()
        .expect("Filtered Calendar not found in context");
    let active_filter =
        use_context::<RwSignal<Option<Filter>>>().expect("Filter not found in context");

    let options: Signal<Vec<SelectOption<Filter>>> = Signal::derive(move || {
        filters
            .get()
            .unwrap_or_default()
            .into_iter()
            .map(|(name, filter)| SelectOption::new(name, filter))
            .collect()
    });

    view! {
        { move ||  {
            let _  = active_filter.get();
            update_calendar(calendar, filtered_calendar, active_filter, false);
        }
        }

        <Layout>
            <Space align=SpaceAlign::Center>
                <Text>"Active filter"</Text>
                <Select value=active_filter options />
            </Space>
            <p>"filter: " {move || format!("{:#?}", active_filter.get())}</p>
        </Layout>
    }
}

/// Ask the server for the plannings of the specified days
/// This does not reload already loaded days
pub(crate) fn update_calendar(
    calendar: RwSignal<Calendar>,
    filtered_calendar: RwSignal<FilteredCalendar>,
    filter: RwSignal<Option<Filter>>,
    load_new_batch: bool,
) {
    if load_new_batch {
        // Closure to update the calendar
        calendar.update(|cal| {
            cal.load_batch();
        })
    };
    filtered_calendar.set(FilteredCalendar::new(
        calendar.get_untracked(),
        filter.get_untracked(),
    ));
    trace!(
        "Updated calendar: {:?}",
        calendar.get_untracked().days.keys()
    );
}
