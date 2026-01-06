use crate::logic::calendar_ui::{Calendar, CalendarDayState, DayPlanning, FilteredCalendar};
use crate::views::book_court::{day_availability::DayAvailaibilityList, update_calendar};
use chrono::{DateTime, Days, Local, Weekday};
use leptos::*;
use shared::utils::get_next_days_from;
use shared::{filter::Filter, DATE_FORMAT, DAYS_PER_WEEK, NB_DAYS_PER_BATCH};
use thaw::*;
use tracing::*;

#[component]
pub fn AvailaibilityCalendar() -> impl IntoView {
    let calendar = use_context::<RwSignal<Calendar>>().unwrap();
    let filtered_calendar = use_context::<RwSignal<FilteredCalendar>>()
        .expect("Filtered Calendar not found in context");
    let filter = use_context::<RwSignal<Option<Filter>>>().expect("Filter not found in context");

    // Days shown by the UI
    let (days_shown, set_days_shown) = create_signal::<Vec<Vec<DateTime<Local>>>>(vec![]);
    let show = create_rw_signal(false);

    // Planning for selected day
    let (planning, set_planning) = create_signal((None, DayPlanning::default()));

    // Init by retrieving calendar from server
    let now_datetime = chrono::Local::now();
    set_days_shown.update(|days_shown| *days_shown = get_next_days_from(now_datetime));

    view! {
        <div id="availability-calendar-prev-next-wrapper">
        <div id="availability-calendar-prev">
        <Button
        // Show previous days
        on:click=move |_| {
        set_days_shown
            .update(|days_shown| {
                let first_day_shown = days_shown[0][0];
                let next_first_day_shown = first_day_shown
                    .checked_sub_days(Days::new(NB_DAYS_PER_BATCH as u64))
                    .unwrap();
                *days_shown = get_next_days_from(next_first_day_shown);
            });
    }
        color=ButtonColor::Warning
        >
        Prev
        </Button>
        </div>
        <div id="availability-calendar-next">
        <Button
        // Show next days
        on:click=move |_| {
            set_days_shown
                .update(|days_shown| {
                    let first_day_shown = days_shown[0][0];
                    let next_first_day_shown = first_day_shown
                        .checked_add_days(Days::new(NB_DAYS_PER_BATCH as u64))
                        .unwrap();
                    *days_shown = get_next_days_from(next_first_day_shown);
                });
            let load_new_batch = days_shown.with_untracked(|next_days_shown| {
                let last_day_shown = next_days_shown.last().unwrap().last().unwrap();
                debug!("last day shown: {:?}", last_day_shown);
                calendar.with_untracked(|cal| last_day_shown > &chrono::Local::now() && !cal.days.contains_key(&last_day_shown.format(DATE_FORMAT).to_string()))
            });
            if load_new_batch {
                update_calendar(calendar, filtered_calendar, filter, true);
            }
        }
            color=ButtonColor::Warning
                >
                Next
                </Button>
                </div>
                </div>
                <Table>
                <div id="availability-calendar-headers">
                {(0..DAYS_PER_WEEK)
                    .map(|weekday| {
                        view! {
                            <div class="availability-calendar-headers-cell">
                            {Weekday::try_from(weekday).unwrap().to_string()}
                            </div>
                        }
                    })
                    .collect_view()}

                </div>
                <div id="availability-calendar-body">
                {move || {
                    days_shown
                        .get()
                        .into_iter()
                        .map(|week| {
                            view! {
                                <div class="calendar-week-row">
                                {week
                                    .into_iter()
                                    .map(|day| {
                                        view! {
                                            <div
                                            class="calendar-day-cell"
                                                class=("calendar-day-cell-past", move || now_datetime > day)
                                                class=(
                                                    "calendar-day-cell-today",
                                                    move || now_datetime == day,
                                                )
                                                >
                                                {move || {
                                                    let day_string = day.format(DATE_FORMAT).to_string();
                                                    let calendar = filtered_calendar.get();
                                                    let day_text = { format!("{}", day.format("%d - %b")) };
                                                    match calendar.get(&day_string) {
                                                        CalendarDayState::Loaded(day_planning) => {
                                                            let dp_clone = day_planning.clone();
                                                            view! {
                                                                <Button
                                                                color=ButtonColor::Primary
                                                                on_click=move |_| {
                                                                    set_planning
                                                                        .set({
                                                                            trace!(
                                                                                "Selected day: {:?} {:?}",dp_clone.weekday, &day_string
                                                                            );
                                                                            (Some(day_string.to_string()), dp_clone.clone())
                                                                        });
                                                                    show.set(true);
                                                                }
                                                                >
                                                                {day_text}
                                                                </Button>
                                                            }
                                                        }
                                                        CalendarDayState::Loading() => {
                                                            view! { <Spinner size=SpinnerSize::Medium /> }
                                                        }
                                                        CalendarDayState::NoAvailaibility() => {
                                                            view! {
                                                                <Button color=ButtonColor::Error>{day_text}</Button>
                                                            }
                                                        }
                                                        CalendarDayState::NotLoaded() => {
                                                            if day < chrono::Local::now() {
                                                                view! {
                                                                    <Button variant=ButtonVariant::Primary disabled=true>
                                                                    {day_text}
                                                                    </Button>
                                                                }
                                                            } else {
                                                                view! { <Spinner size=SpinnerSize::Medium /> }
                                                            }
                                                        }
                                                    }
                                                }}
                                                </div>
                                        }
                                    })
                                    .collect_view()}
                                </div>
                            }
                        })
                        .collect_view()
                }}
                </div>
                </Table>
                <Modal show>
                <DayAvailaibilityList planning=planning />
                </Modal>
        }
}
