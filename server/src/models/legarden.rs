//! Structure to parse JSON response from DoinSport server

use std::{collections::BTreeMap, fmt::Debug, ops::Deref, slice::Iter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Availabilities(pub BTreeMap<String, DayPlanningResponse>);

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DayPlanningResponse {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    at_type: String,
    #[serde(rename = "hydra:member")]
    courts: Vec<Court>,
    #[serde(rename = "hydra:totalItems")]
    total_items: usize,
    #[serde(rename = "hydra:view")]
    view: View,
    // #[serde(rename = "hydra:search")]
    // search: Search,
}

#[cfg(feature = "local_dev")]
impl DayPlanningResponse {
    pub fn simple_day() -> Self {
        serde_json::from_str(&testcases::legarden::json_planning_simple_day()).unwrap()
    }
}

impl Debug for DayPlanningResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DayPlanningResponse")
            .field("courts", &self.courts)
            .finish()
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Court {
    #[serde(rename = "@type")]
    at_type: String,
    #[serde(rename = "@id")]
    at_id: String,
    id: String,
    name: String,
    indoor: bool,
    surface: String,
    closures: Vec<()>,
    activities: Vec<CourtActivity>,
    timetables: TimeTable,
}

impl Debug for Court {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PadelCourtResponse")
            .field("indoor", &self.indoor)
            .field("activities", &self.activities)
            .finish()
    }
}

impl Default for Court {
    fn default() -> Self {
        Self {
            at_type: Default::default(),
            at_id: Default::default(),
            id: Default::default(),
            name: Default::default(),
            indoor: Default::default(),
            surface: Default::default(),
            closures: Default::default(),
            activities: vec![CourtActivity::default()],
            timetables: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CourtActivity {
    id: String,
    name: String,
    slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    start_at: String,
    payment_methods: Vec<String>,
    instalment_percentage: Option<()>,
    prices: Vec<Price>,
    user_client_step_booking_duration: usize,
}

impl Debug for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slot")
            .field("start_at", &self.start_at)
            .field("prices", &self.prices)
            .finish()
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    id: String,
    duration: usize,
    price_per_participant: usize,
    participant_count: usize,
    instalment_amount: Option<()>,
    bookable: bool,
}

impl Debug for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Price")
            .field("duration", &self.duration)
            .field("bookable", &self.bookable)
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeTable {
    start_at: String,
    end_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct View {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    at_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    #[serde(rename = "@type")]
    at_type: String,
    #[serde(rename = "hydra:template")]
    template: String,
    #[serde(rename = "hydra:variableRepresentation")]
    variable_representation: String,
    #[serde(rename = "hydra:mapping")]
    mapping: Vec<Mapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    #[serde(rename = "@type")]
    at_type: String,
    variable: String,
    property: String,
    required: bool,
}

impl DayPlanningResponse {
    pub fn courts(&self) -> &Vec<Court> {
        &self.courts
    }

    pub fn courts_mut(&mut self) -> &mut Vec<Court> {
        &mut self.courts
    }

    // Get date from url request that looks like this:
    // "/clubs/playgrounds/plannings/2024-10-04?club.id=a126b4d4..."
    pub fn date(&self) -> &str {
        const BASE_REQ: &str = "/clubs/playgrounds/plannings/";
        const DATE_LEN: usize = 10;
        let url = &self.view.id;
        let remove_first_n = BASE_REQ.len();
        &url[remove_first_n..remove_first_n + DATE_LEN]
    }

    pub fn new_with(&self, courts: Vec<Court>) -> Self {
        DayPlanningResponse {
            context: self.context.clone(),
            id: self.id.clone(),
            at_type: self.at_type.clone(),
            courts,
            total_items: self.total_items,
            view: self.view.clone(),
        }
    }
}

impl Court {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_indoor(&self) -> bool {
        self.indoor
    }

    pub fn set_indoor(&mut self, indoor: bool) {
        self.indoor = indoor;
    }

    pub fn slots(&self) -> &Vec<Slot> {
        &self.activities[0].slots
    }

    pub fn slots_mut(&mut self) -> &mut Vec<Slot> {
        &mut self.activities[0].slots
    }
    pub fn clone_with(&self, slots: Vec<Slot>) -> Self {
        let mut new = self.clone();
        new.activities[0].slots = slots;
        new
    }
}

impl Slot {
    pub fn start_at(&self) -> &str {
        &self.start_at
    }

    pub fn start_at_mut(&mut self) -> &str {
        &mut self.start_at
    }

    pub fn prices(&self) -> &Vec<Price> {
        &self.prices
    }

    pub fn prices_mut(&mut self) -> &mut Vec<Price> {
        &mut self.prices
    }

    pub fn clone_with_prices(&self, prices: Vec<Price>) -> Self {
        let mut new = self.clone();
        new.prices = prices;
        new
    }

    pub fn clone_with_start_at(&self, start_at: String) -> Self {
        let mut new = self.clone();
        new.start_at = start_at;
        new
    }
}

impl Price {
    pub fn duration(&self) -> usize {
        self.duration
    }

    pub fn bookable(&self) -> bool {
        self.bookable
    }

    pub fn set_bookable(&mut self, bookable: bool) {
        self.bookable = bookable;
    }
}

#[derive(Debug)]
pub struct AvailIter<'a> {
    cur_day: Option<(&'a String, &'a DayPlanningResponse)>,
    cur_court: Option<&'a Court>,
    cur_slot: Option<&'a Slot>,
    cur_price: Option<&'a Price>,
    days: std::collections::btree_map::Iter<'a, String, DayPlanningResponse>,
    courts: Option<Iter<'a, Court>>,
    slots: Option<Iter<'a, Slot>>,
    prices: Option<Iter<'a, Price>>,
}

impl Availabilities {
    pub fn iter(&self) -> AvailIter<'_> {
        AvailIter {
            days: self.0.iter(),
            courts: None,
            slots: None,
            prices: None,
            cur_day: None,
            cur_court: None,
            cur_slot: None,
            cur_price: None,
        }
    }
}

impl<'a> Iterator for AvailIter<'a> {
    type Item = (&'a String, &'a Court, &'a Slot, &'a Price);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // dbg!(&self);
            if let Some(prices) = &mut self.prices {
                if let Some(price) = prices.next() {
                    // We are iterating over prices
                    self.cur_price = Some(price);
                    return Some((
                        self.cur_day?.0,
                        self.cur_court?,
                        self.cur_slot?,
                        self.cur_price?,
                    ));
                }
            }

            // We have finished iterating over prices or we are starting
            if let Some(slots) = &mut self.slots {
                if let Some(slot) = slots.next() {
                    // We are iterating over slots
                    self.cur_slot = Some(slot);
                    self.prices = Some(slot.prices().iter());
                    continue;
                }
            }
            // We have finished iterating over slots or we are starting
            if let Some(courts) = &mut self.courts {
                if let Some(court) = courts.next() {
                    // We are iterating over courts
                    self.cur_court = Some(court);
                    self.slots = Some(court.slots().iter());
                    continue;
                }
            }

            // We have finished iterating over courts or we are starting
            if let Some((date, day_planning)) = self.days.next() {
                self.cur_day = Some((date, day_planning));
                self.courts = Some(day_planning.courts().iter());
                continue;
            }

            // We have finished iterating over days
            return None;
        }
    }
}

impl Deref for Availabilities {
    type Target = BTreeMap<String, DayPlanningResponse>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Availabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Availabilities [\n")?;
        for (date, court, slot, price) in self.iter() {
            write!(
                f,
                "{} {} | \"{}\": {} bookable:{}\n",
                date,
                slot.start_at(),
                court.name(),
                price.duration(),
                price.bookable
            )?
        }
        write!(f, "]")
    }
}

#[cfg(all(test, feature = "local_dev"))]
mod tests {
    use testcases::legarden::json_planning_simple_day;

    #[test]
    fn test_availabilities_iter() {
        let avail = crate::mock::simple_availabilities(4, json_planning_simple_day());
        assert_eq!(avail.iter().count(), 4 * 3);
        dbg!(&avail);
        let expected_duration = [3600, 7200, 5400];
        for (i, (_date, court, slot, price)) in avail.iter().enumerate() {
            assert_eq!(slot.start_at(), "10:00");
            assert_eq!(court.name(), "Padel 1");
            assert_eq!(price.duration(), expected_duration[i % 3])
        }
    }
}
