use crate::logic::calendar_ui::{Calendar, CalendarDayState, FilteredCalendar};
use crate::views::book_court::{day_availability::DayAvailaibilityItem, update_calendar};
use leptos::*;
use shared::filter::Filter;
use thaw::*;
use tracing::*;

#[component]
pub fn NextCourtsView() -> impl IntoView {
    let calendar = use_context::<RwSignal<Calendar>>().unwrap();
    let filtered_calendar = use_context::<RwSignal<FilteredCalendar>>()
        .expect("Filtered Calendar not found in context");
    let filter = use_context::<RwSignal<Option<Filter>>>().expect("Filter not found in context");

    view! {
            <For
            each=move || filtered_calendar.get().calendar.into_keys()
                key=|date| date.clone()
                    let:date>
                    {move || {
                        match filtered_calendar.get().get(&date) {
                            CalendarDayState::Loading() => {
                                view! { <Spinner size=SpinnerSize::Medium /> }
                            },
                            CalendarDayState::NotLoaded() => {
                                view! { <Spinner size=SpinnerSize::Medium /> }
                            },
                            CalendarDayState::NoAvailaibility() => {
                                view! {}.into_view()
                            },
                            CalendarDayState::Loaded(mut day_planning) => {
                                let date_clone = date.clone();
                                view! {
                                    <Table style="--thaw-border-color: black; --thaw-background-color: #FDE992">
                                    <tbody>

                                    <tr>
                                    <td style="padding: 0px">
                                    {
                                        let (time, slot) = day_planning.slots.first_entry().unwrap().remove_entry();
                                        let weekday = day_planning.weekday;
                                        view! {
                                            <Layout>
                                            <LayoutHeader style="background-color: #E3B778; padding: 0.5em; font-size: large; font-weight: bold;">
    {weekday} " "{date_clone}
                                        </LayoutHeader>
                                        <Layout>
                                        <DayAvailaibilityItem time=time.clone() sl=slot.clone() />
                                        </Layout>
                                        </Layout>
                                    }
                                    }
                                    </td>
                                    </tr>
                                    </tbody>
                                    </Table>
                                }
                            }
                        }
                    }
                    }
                    </For>

                    <Layout>
                    <Layout>
                    <Button
                    style="width:100%;"
                    on_click=move |_| {
                        update_calendar(calendar, filtered_calendar, filter, true);
                    }
                    >
                    "Load more courts"
                    </Button>
                    </Layout>
                    </Layout>

        }
}
