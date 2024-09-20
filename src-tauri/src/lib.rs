mod server_calls;
use server_calls::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let future = test_reqwest();
    // rt.block_on(future);
    tauri::Builder::default()
        // .plugin(tauri_plugin_fs::init())
        // .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_planning, get_local_planning])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// async fn test_reqwest() {
//     let result = reqwest::get("https://api.spotify.com/v1/search").await;
//     println!("{:?}", result);
// }
