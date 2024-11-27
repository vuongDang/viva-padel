use crate::book_court::{
    day_availability::DayAvailaibilityList, update_calendar, FilteredCalendar,
};
use chrono::{DateTime, Days, Local, Weekday};
use leptos::*;
use shared::frontend::{
    calendar_ui::{Calendar, DayPlanning},
    utils::{flatten_days, get_next_days_from},
};
use shared::{DATE_FORMAT, DAYS_PER_WEEK, NB_DAYS_SHOWN};
use thaw::*;
use tracing::*;

#[component]
pub fn AvailaibilityCalendar(
    calendar: RwSignal<Calendar>,
    filtered_calendar: FilteredCalendar,
) -> impl IntoView {
    // Days shown by the UI
    let (days_shown, set_days_shown) = create_signal::<Vec<Vec<DateTime<Local>>>>(vec![]);
    let show = create_rw_signal(false);

    // Planning for selected day
    let (planning, set_planning) = create_signal((None, DayPlanning::default()));

    // Init by retrieving calendar from server
    let now_datetime = chrono::Local::now();
    set_days_shown.update(|days_shown| *days_shown = get_next_days_from(now_datetime));

    update_calendar(calendar, flatten_days(days_shown.get_untracked()));

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
                                    .checked_sub_days(Days::new(NB_DAYS_SHOWN as u64))
                                    .unwrap();
                                *days_shown = get_next_days_from(next_first_day_shown);
                            });
                        update_calendar(calendar, flatten_days(days_shown.get()));
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
                                    .checked_add_days(Days::new(NB_DAYS_SHOWN as u64))
                                    .unwrap();
                                *days_shown = get_next_days_from(next_first_day_shown);
                            });
                        update_calendar(calendar, flatten_days(days_shown.get()));
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
                                                        let dp_signal: Option<&Signal<Option<DayPlanning>>> = calendar
                                                            .get(&day_string);
                                                        let day_planning: Option<DayPlanning> = dp_signal
                                                            .map(|signal| signal.get())
                                                            .unwrap_or_default();
                                                        match day_planning {
                                                            None => {
                                                                view! { <Spinner size=SpinnerSize::Medium /> }
                                                            }
                                                            Some(day_planning) => {
                                                                let dp_clone = day_planning.clone();
                                                                view! {
                                                                    <Button
                                                                        color=Signal::derive(move || {
                                                                            if day_planning.slots.is_empty()
                                                                                || day_planning.weekday
                                                                                    < chrono::Local::now().format(DATE_FORMAT).to_string()
                                                                            {
                                                                                ButtonColor::Error
                                                                            } else {
                                                                                ButtonColor::Primary
                                                                            }
                                                                        })
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
                                                                        {format!("{}", day.format("%d - %b"))}
                                                                    </Button>
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
