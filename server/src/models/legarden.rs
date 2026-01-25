//! Structure to parse JSON response from DoinSport server

use std::{collections::BTreeMap, fmt::Debug};

use serde::{Deserialize, Serialize};
pub type Availabilities = BTreeMap<String, DayPlanningResponse>;

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
    courts: Vec<PadelCourtResponse>,
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
pub struct PadelCourtResponse {
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

impl Debug for PadelCourtResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PadelCourtResponse")
            .field("indoor", &self.indoor)
            .field("activities", &self.activities)
            .finish()
    }
}

impl Default for PadelCourtResponse {
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
    pub fn courts(&self) -> &Vec<PadelCourtResponse> {
        &self.courts
    }

    pub fn courts_mut(&mut self) -> &mut Vec<PadelCourtResponse> {
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

    pub fn new_with(&self, courts: Vec<PadelCourtResponse>) -> Self {
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

impl PadelCourtResponse {
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
