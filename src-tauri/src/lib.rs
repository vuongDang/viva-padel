#[cfg(not(feature = "local_dev"))]
mod server_calls;
#[cfg(not(feature = "local_dev"))]
use server_calls::*;

#[cfg(feature = "local_dev")]
mod local_dev_server_calls;
#[cfg(feature = "local_dev")]
use local_dev_server_calls::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let future = test_reqwest();
    // rt.block_on(future);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_date_planning])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// async fn test_reqwest() {
//     let result = reqwest::get("https://api.spotify.com/v1/search").await;
//     println!("{:?}", result);
// }
