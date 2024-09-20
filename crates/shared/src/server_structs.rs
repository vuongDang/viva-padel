//! Structure to parse JSON response from DoinSport server

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourtActivity {
    id: String,
    name: String,
    slots: Vec<Slot>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    start_at: String,
    payment_methods: Vec<String>,
    instalment_percentage: Option<()>,
    prices: Vec<Price>,
    user_client_step_booking_duration: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    id: String,
    duration: usize,
    price_per_participant: usize,
    participant_count: usize,
    instalment_amount: Option<()>,
    bookable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeTable {
    start_at: String,
    end_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    at_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    #[serde(rename = "@type")]
    at_type: String,
    variable: String,
    property: String,
    required: bool,
}

impl DayPlanningResponse {
    pub(crate) fn courts(&self) -> &Vec<PadelCourtResponse> {
        &self.courts
    }

    // Get date from url request that looks like this:
    // "/clubs/playgrounds/plannings/2024-10-04?club.id=a126b4d4..."
    pub(crate) fn day(&self) -> &str {
        const BASE_REQ: &str = "/clubs/playgrounds/plannings/";
        const DATE_LEN: usize = 10;
        let url = &self.view.id;
        let remove_first_n = BASE_REQ.len();
        &url[remove_first_n..remove_first_n + DATE_LEN]
    }
}

impl PadelCourtResponse {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn is_indoor(&self) -> bool {
        self.indoor
    }

    pub(crate) fn slots(&self) -> &Vec<Slot> {
        &self.activities[0].slots
    }
}

impl Slot {
    pub(crate) fn start_at(&self) -> &str {
        &self.start_at
    }

    pub(crate) fn prices(&self) -> &Vec<Price> {
        &self.prices
    }
}

impl Price {
    pub(crate) fn duration(&self) -> usize {
        self.duration
    }

    pub(crate) fn bookable(&self) -> bool {
        self.bookable
    }
}