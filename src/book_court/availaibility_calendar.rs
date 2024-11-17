use crate::book_court::day_availability::DayAvailaibilityList;
use chrono::{DateTime, Days, Local, Weekday};
use leptos::*;
use shared::frontend::{calendar_ui::{DayPlanning, Filter, Calendar, DateKey}, utils::{get_next_days_from, flatten_days}};
use std::collections::BTreeMap;
use shared::{DATE_FORMAT, NB_DAYS_SHOWN, DAYS_PER_WEEK};
use thaw::*;

#[component]
pub fn AvailaibilityCalendar(filter: RwSignal<Filter>) -> impl IntoView {
    // Days shown by the UI
    let (days_shown, set_days_shown) = create_signal::<Vec<Vec<DateTime<Local>>>>(vec![]);
    let show = create_rw_signal(false);

    // Court availabilities for all the days loaded 
    let calendar: RwSignal<Calendar> = create_rw_signal(Calendar::new());

    // Planning for selected day
    let (planning, set_planning) = create_signal((None, DayPlanning::default()));

    // Init by retrieving calendar from server
    let now_datetime = chrono::Local::now();
    set_days_shown.update(|days_shown| *days_shown = get_next_days_from(now_datetime));

    // Closure to update the calendar
    let update_calendar = move  || {
        let flatten_days = flatten_days(days_shown.get_untracked());
        calendar.update(|cal| {
            for day_shown  in flatten_days.into_iter() {
                cal.days.entry(day_shown.clone()).or_insert(create_resource(
                    || (),
                    move |_| {
                        let day_shown_clone = day_shown.clone();
                        async move { 
                            let d = DayPlanning::retrieve(&day_shown_clone).await ;
                            d
                        }
                    }
                ));
            }
        });
        logging::log!("Updated calendar: {:?}",calendar.get_untracked().days.keys());
    };
    update_calendar();

    // The calendar that is displayed after applying the filter
    let filtered_calendar: Signal<BTreeMap<DateKey, DayPlanning>>   = Signal::derive(move || {
        // leptos::logging::log!("Filtering!");
        calendar.get().filtered(&filter.get())
    });


    view! {
        <span hidden>{move || format!("{:?}", filtered_calendar.get())}</span>
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
                        update_calendar();
                        leptos::logging::log!("Prev!");
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
                        update_calendar();
                        leptos::logging::log!("Next!");
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
                                                    <Transition fallback=move || {
                                                        view! { <Spinner size=SpinnerSize::Medium /> }
                                                    }>
                                                        <Button
                                                            // Disabled if no courts are available on this day
                                                            // disabled=Signal::derive(move || {
                                                            // let day_string = day.format(DATE_FORMAT).to_string();
                                                            // let calendar = filtered_calendar.get();
                                                            // let day_planning = calendar.get(&day_string);
                                                            // day_planning.is_none()
                                                            // || day_planning.unwrap().slots.is_empty()
                                                            // || day_planning.unwrap().weekday
                                                            // < chrono::Local::now().format(DATE_FORMAT).to_string()
                                                            // })
                                                            color=Signal::derive(move || {
                                                                let day_string = day.format(DATE_FORMAT).to_string();
                                                                let calendar = filtered_calendar.get();
                                                                let day_planning = calendar.get(&day_string);
                                                                if day_planning.is_none()
                                                                    || day_planning.unwrap().slots.is_empty()
                                                                    || day_planning.unwrap().weekday
                                                                        < chrono::Local::now().format(DATE_FORMAT).to_string()
                                                                {
                                                                    ButtonColor::Error
                                                                } else {
                                                                    ButtonColor::Primary
                                                                }
                                                            })
                                                            on_click=move |_| {
                                                                let date_string = day.format(DATE_FORMAT).to_string();
                                                                set_planning
                                                                    .set({
                                                                        let calendar = filtered_calendar.get();
                                                                        let day_planning = calendar
                                                                            .get(&date_string)
                                                                            .expect("Selected day not in the calendar");
                                                                        logging::log!(
                                                                            "Selected day: {:?} {:?}",day_planning.weekday, date_string
                                                                        );
                                                                        (Some(date_string), day_planning.clone())
                                                                    });
                                                                show.set(true);
                                                            }
                                                        >
                                                            {format!("{}", day.format("%d - %b"))}
                                                        </Button>
                                                    </Transition>
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
        <div style="margin-top: 1rem">
            <p>"filter: " {move || format!("{:#?}", filter.get())}</p>
            <p>
                {move || {
                    let day_planning = if let Some(resource) = calendar
                        .get()
                        .days
                        .get(&planning.get().0.unwrap_or_default())
                    {
                        resource.get()
                    } else {
                        Some(DayPlanning::default())
                    };
                    format!("{:?}", day_planning)
                }}
            </p>
            <p>{move || { format!("{:#?}", planning.get()) }}</p>
        // <Button
        // // Show previous days
        // on:click=move |_| {
        // calendar.get().filtered(&filter.get());
        // }
        // color=ButtonColor::Warning
        // >
        // "Filter"
        // </Button>
        </div>
    }
}
