// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

async fn get_from_kiara_api(path: &str) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    client
        .get(format!("http://localhost:8080/{path}"))
        .send()
        .await
        .map_err(|e| {
            format!(
                "Failed to connect to Kiara API: {e} Did you forget to start your Kiara server?"
            )
        })?
        .error_for_status()
        .map_err(|e| format!("HTTP error in response from kiara: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Failed to decode response from kiara: {e}"))
}

#[tauri::command]
async fn query_kiara() -> Result<serde_json::Value, String> {
    get_from_kiara_api("modules/type_names").await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, query_kiara])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
