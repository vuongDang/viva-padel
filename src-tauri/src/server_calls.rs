use shared_structs::server_error::Error;
use shared_structs::server_structs::*;

const BASE_URL: &str = "https://api-v3.doinsport.club/clubs/playgrounds";
const CLUB_ID: &str = "club.id=a126b4d4-a2ee-4f30-bee3-6596368368fb";
const PADEL_ID: &str = "activities.id=ce8c306e-224a-4f24-aa9d-6500580924dc";

/// Call to get the planning of available fields
// Example request: 'https://api-v3.doinsport.club/clubs/playgrounds/plannings/2024-07-22?club.id=a126b4d4-a2ee-4f30-bee3-6596368368fb&from=21:30&to=23:59:59&activities.id=ce8c306e-224a-4f24-aa9d-6500580924dc&bookingType=unique'
#[tauri::command]
pub(crate) async fn get_planning(day: String) -> Result<PlanningResponse, Error> {
    let request = format!(
        "{BASE_URL}/plannings/{}?{CLUB_ID}&from={}&to={}&{PADEL_ID}&bookingType=Unique",
        day, "10:00", "23:00"
    );
    let req_result = reqwest::get(request).await?.text().await?;
    let parsed = serde_json::from_str::<PlanningResponse>(&req_result)?;
    Ok(parsed)
}
