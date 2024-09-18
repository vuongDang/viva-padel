use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanningResponse {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    at_type: String,
    #[serde(rename = "hydra:member")]
    courts: Vec<PadelCourt>,
    #[serde(rename = "hydra:totalItems")]
    total_items: usize,
    // #[serde(rename = "hydra:view")]
    // view: View,
    // #[serde(rename = "hydra:search")]
    // search: Search,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PadelCourt {
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
