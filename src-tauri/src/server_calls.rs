use shared::app_structs::*;
use shared::errors::Error;
use shared::server_structs;
use std::{fs::File, io::Write, path::PathBuf};

const BASE_URL: &str = "https://api-v3.doinsport.club/clubs/playgrounds";
const CLUB_ID: &str = "club.id=a126b4d4-a2ee-4f30-bee3-6596368368fb";
const PADEL_ID: &str = "activities.id=ce8c306e-224a-4f24-aa9d-6500580924dc";
const OPENING_HOUR: &str = "08:00";
const CLOSING_HOUR: &str = "23:00";

/// Call to get the planning of available fields
// Example request: 'https://api-v3.doinsport.club/clubs/playgrounds/plannings/2024-07-22?club.id=a126b4d4-a2ee-4f30-bee3-6596368368fb&from=21:30&to=23:59:59&activities.id=ce8c306e-224a-4f24-aa9d-6500580924dc&bookingType=unique'
#[tracing::instrument]
#[tauri::command]
pub(crate) async fn get_date_planning(date: String) -> Result<DayPlanningResponse, Error> {
    let request = format!(
        "{BASE_URL}/plannings/{date}?{CLUB_ID}&from={OPENING_HOUR}&to={CLOSING_HOUR}&{PADEL_ID}&bookingType=Unique",
    );
    let req_result: String = reqwest::get(request).await?.text().await?;
    let parsed = serde_json::from_str::<server_structs::DayPlanningResponse>(&req_result)?;
    // let path = PathBuf::from(format!("tests\\json_responses\\get_planning\\{date}.json"));
    // print_to_test_file(path, req_result);
    Ok(parsed)
}

// Used to store results to disk for local development
fn print_to_test_file(path: PathBuf, content: String) -> std::io::Result<()> {
    let dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = std::env::current_dir()?;
    let mut full_path = PathBuf::from(dir);
    full_path.push(path);
    let mut f = File::create(full_path.clone())?;
    f.write_all(content.as_bytes()).unwrap();
    println!("Wrote to {:?}", full_path);
    Ok(())
}
