mod availaibility_calendar;
mod day_availability;
mod filter_builder;
mod next_courts;

use crate::book_court::availaibility_calendar::AvailaibilityCalendar;
use crate::book_court::filter_builder::FilterView;
use crate::book_court::next_courts::NextCourtsView;
use std::collections::{BTreeMap, HashMap};
use leptos::*;
use shared::frontend::calendar_ui::{Calendar, Filter, DayPlanning, DateKey};
use thaw::mobile::*;
use thaw::*;
use tracing::*;

pub(crate) type FilteredCalendar = RwSignal<BTreeMap<String, Signal<Option<DayPlanning>>>>;
const DEFAULT_TAB: &str = "next_courts";

#[component]
pub fn BookCourtView() -> impl IntoView {
    let selected_tab = create_rw_signal(String::from(DEFAULT_TAB));
    let stored_filters = Resource::once(|| async move {Filter::get_stored_filters().await.expect("Failed to retrieved filters stored on disk")});
    let filters = create_rw_signal(None);


    // The active filter, the option type is solely to fit the selector item from Thaw UI
    let active_filter = create_rw_signal(Some(Filter::default()));

    // Court availabilities for all the days loaded
    let calendar: RwSignal<Calendar> = create_rw_signal(Calendar::new());

    // The calendar obtained after applying the filter
    let filtered_calendar:  FilteredCalendar = create_rw_signal(
        // trace!("Compute new [`filtered_calendar`]");
        calendar.get().days.clone().into_iter().map(|(date, dp_resource)| {
            (date, 
                Signal::derive(move || { 
                    dp_resource.get().map(|dp| DayPlanning::filtered(&dp, &active_filter.get().unwrap()))
                })
            )
        }).collect()
    );


    view! {
        <Tabs value=selected_tab>
            {move || match stored_filters.get() {
                None => view! { <p>"Toto"</p> }.into_view(),
                Some(stored_filters) => {
                    filters.set(Some(stored_filters));
                    view! { <FilterSelector active_filter filters /> }
                }
            }} <Tab key="filter" label="Filter">
                <FilterView active_filter filters />
            // <Tab key="calendar" label="Calendar">
            // <AvailaibilityCalendar calendar filtered_calendar />
            // </Tab>
            </Tab> <Tab key="next_courts" label="Next Courts">
                <NextCourtsView calendar filtered_calendar />
            </Tab>
        </Tabs>
    }
}


#[component]
pub(crate) fn FilterSelector(active_filter: RwSignal<Option<Filter>>, filters: RwSignal<Option<HashMap<String, Filter>>>) -> impl IntoView {
let options: Signal<Vec<SelectOption<Filter>>> = Signal::derive(move || 
        filters.get().unwrap_or_default().into_iter().map(|(name, filter)| SelectOption::new(name, filter)).collect());
    view! {
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
pub(crate) fn update_calendar(calendar: RwSignal<Calendar>, dates: Vec<String>) {
    // Closure to update the calendar
    calendar.update(|cal| {
        for day_shown  in dates.into_iter() {
            cal.days.entry(day_shown.clone()).or_insert(create_resource(
                || (),
                move |_| {
                    let day_shown_clone = day_shown.clone();
                    async move { 
                        let res = DayPlanning::retrieve(&day_shown_clone).await.expect("Failed to retrieve new days to update calendar") ;
                        trace!("Finished retrieving {:?}", day_shown_clone);
                        res
                    }
                }
            ));
        }
    });
    trace!("Updated calendar: {:?}",calendar.get_untracked().days.keys());
}

