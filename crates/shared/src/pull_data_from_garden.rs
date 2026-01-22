use chrono::NaiveDate;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::models::*;
use crate::{models, DATE_FORMAT};

const BASE_URL: &str = "https://api-v3.doinsport.club/clubs/playgrounds";
const CLUB_ID: &str = "club.id=a126b4d4-a2ee-4f30-bee3-6596368368fb";
const PADEL_ID: &str = "activities.id=ce8c306e-224a-4f24-aa9d-6500580924dc";
const OPENING_HOUR: &str = "08:00";
const CLOSING_HOUR: &str = "23:00";

fn day_planning_url(date: &str) -> String {
    format!(
            "{BASE_URL}/plannings/{date}?{CLUB_ID}&from={OPENING_HOUR}&to={CLOSING_HOUR}&{PADEL_ID}&bookingType=Unique",
        )
}

/// Pull availability data for LeGarden availability
pub async fn get_day_planning(date: &str) -> Result<DayPlanningResponse, Error> {
    let request = day_planning_url(date);
    let req_result: String = reqwest::get(request).await?.text().await?;
    let parsed = serde_json::from_str::<models::DayPlanningResponse>(&req_result)?;
    Ok(parsed)
}

pub const NB_DAYS_IN_BATCH: u64 = 90;

pub fn json_to_calendar(nb_days: u64) -> Availibilities {
    let today = chrono::Local::now().date_naive();
    let next_3_months = batch_dates(today, nb_days);
    let mut calendar = BTreeMap::new();
    let json_planning = testcases::legarden::json_planning_for_29_days();
    for (i, date) in next_3_months.iter().enumerate() {
        let date_str = date.format(DATE_FORMAT).to_string();
        let day_planning: DayPlanningResponse =
            serde_json::from_str(json_planning.get(i % json_planning.len()).unwrap()).unwrap();
        calendar.insert(date_str, day_planning);
    }
    calendar
}

#[cfg(feature = "local_dev")]
pub async fn get_calendar() -> Result<Availibilities, Error> {
    Ok(json_to_calendar(NB_DAYS_IN_BATCH))
}

#[cfg(feature = "local_dev")]
pub async fn get_simple_calendar(booked: bool) -> Result<Availibilities, Error> {
    let today = chrono::Local::now().date_naive();
    let next_3_months = batch_dates(today, 10);
    let mut calendar = BTreeMap::new();
    for (_, date) in next_3_months.iter().enumerate() {
        use testcases::legarden::json_planning_simple_all_booked;
        use testcases::legarden::json_planning_simple_day;
        let date_str = date.format(DATE_FORMAT).to_string();
        let day_planning: DayPlanningResponse = if booked {
            serde_json::from_str(&json_planning_simple_all_booked()).unwrap()
        } else {
            serde_json::from_str(&json_planning_simple_day()).unwrap()
        };
        calendar.insert(date_str, day_planning);
    }
    Ok(calendar)
}

#[cfg(not(feature = "local_dev"))]
pub async fn get_calendar() -> Result<Availibilities, Error> {
    let today = chrono::Local::now().date_naive();
    let next_3_months = batch_dates(today, NB_DAYS_IN_BATCH);
    let mut calendar = BTreeMap::new();
    for date in next_3_months {
        let date_str = date.format(DATE_FORMAT).to_string();
        let day_planning = get_day_planning(&date_str).await?;
        calendar.insert(date_str, day_planning);
    }
    Ok(calendar)
}

fn batch_dates(from: NaiveDate, nb_days: u64) -> Vec<NaiveDate> {
    (0..nb_days)
        .map(|i| {
            from.checked_add_days(chrono::Days::new(i as u64))
                .expect("Failed to add days")
        })
        .collect()
}

/// Gather courts that got freed between old and new availabilities
pub fn freed_courts(new: &Availibilities, old: &Availibilities) -> Availibilities {
    let mut freed = BTreeMap::new();
    let dates = old.keys().filter(|date| new.contains_key(*date));
    for day_str in dates {
        if let Some(new_day) = new.get(day_str) {
            let old_day = old.get(day_str).unwrap();
            let mut courts = vec![];
            for (old_court, new_court) in old_day.courts().iter().zip(new_day.courts().iter()) {
                let mut slots = vec![];
                for (old_slot, new_slot) in old_court.slots().iter().zip(new_court.slots().iter()) {
                    let mut prices = vec![];
                    for (old_price, new_price) in
                        old_slot.prices().iter().zip(new_slot.prices().iter())
                    {
                        if !old_price.bookable() && new_price.bookable() {
                            prices.push(new_price.clone());
                        }
                    }
                    if !prices.is_empty() {
                        slots.push(Slot::clone_with_prices(new_slot, prices));
                    }
                }
                if !slots.is_empty() {
                    courts.push(PadelCourtResponse::clone_with(new_court, slots));
                }
            }
            if !courts.is_empty() {
                freed.insert(
                    day_str.to_owned(),
                    DayPlanningResponse::new_with(new_day, courts),
                );
            }
        }
    }
    freed
}

#[cfg(all(test, feature = "local_dev"))]
mod tests {
    use super::*;

    #[test]
    fn freed_courts_correct_simple() {
        let mut old_day = DayPlanningResponse::default();
        let mut new_day = DayPlanningResponse::default();

        let mut old_price = Price::default();
        old_price.set_bookable(false);
        let mut old_slot = Slot::default();
        let mut old_court = PadelCourtResponse::default();

        let mut new_price = Price::default();
        new_price.set_bookable(true);
        let mut new_slot = Slot::default();
        let mut new_court = PadelCourtResponse::default();

        old_slot.prices_mut().push(old_price);
        old_court.slots_mut().push(old_slot);
        old_day.courts_mut().push(old_court);

        new_slot.prices_mut().push(new_price);
        new_court.slots_mut().push(new_slot);
        new_day.courts_mut().push(new_court);

        let mut old_cal = BTreeMap::new();
        let mut new_cal = BTreeMap::new();
        old_cal.insert("toto".to_owned(), old_day);
        new_cal.insert("toto".to_owned(), new_day);

        let freed = freed_courts(&new_cal, &old_cal);
        assert_eq!(freed.len(), 1);
    }

    #[test]
    fn freed_courts_correct() {
        let mut old = json_to_calendar(4);
        let mut new = old.clone();
        let new_ptr: *const Availibilities = &new as *const Availibilities;
        let old_ptr: *const Availibilities = &old as *const Availibilities;

        let new_avail = freed_courts(&new, &old);
        assert!(new_avail.is_empty());

        let mut counter = 0;
        for (old_day, new_day) in old.values_mut().zip(new.values_mut()) {
            for (old_court, new_court) in old_day
                .courts_mut()
                .iter_mut()
                .zip(new_day.courts_mut().iter_mut())
            {
                for (old_slot, new_slot) in old_court
                    .slots_mut()
                    .iter_mut()
                    .zip(new_court.slots_mut().iter_mut())
                {
                    for (old_price, new_price) in old_slot
                        .prices_mut()
                        .iter_mut()
                        .zip(new_slot.prices_mut().iter_mut())
                    {
                        old_price.set_bookable(false);
                        new_price.set_bookable(true);
                        counter += 1;
                        unsafe {
                            let avail = freed_courts(&*new_ptr, &*old_ptr);
                            assert_eq!(get_available_prices(&avail).len(), counter);
                        }
                    }
                }
            }
        }
    }

    fn get_available_prices(avail: &Availibilities) -> Vec<Price> {
        let mut prices = vec![];
        avail.values().for_each(|day| {
            day.courts().iter().for_each(|court| {
                court.slots().iter().for_each(|slot| {
                    slot.prices().iter().for_each(|price| {
                        if price.bookable() {
                            prices.push(price.clone());
                        }
                    })
                })
            })
        });
        prices
    }
}
