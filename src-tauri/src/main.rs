// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;

#[tauri::command]
fn greet(name: String) -> String {
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

async fn post_to_kiara_api(
    path: &str,
    body: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    client
        .post(format!("http://localhost:8080/{path}"))
        .json(body)
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

#[tauri::command]
async fn list_operations() -> Result<serde_json::Value, String> {
    let body = json!({
        "input_types": ["network_data"]
    });
    post_to_kiara_api("operations", &body).await
}

#[tauri::command]
async fn list_network_datas() -> Result<serde_json::Value, String> {
    get_from_kiara_api("data/type/network_data/values_info").await
}

#[tauri::command]
async fn import_file(filepath: String) -> Result<serde_json::Value, String> {
    // TODO accept params for first_row_is_header, source column, target column, onboard_type?
    let body = json!({
      "operation_id": "import.file",
      "operation_config": {},
      "inputs": {"source": filepath, "onboard_type": "github"}
    });
    // todo check that actually worked!
    //     check the job is finished and without error
    //     do some error handling if error!
    //     this probably wants to be a run job until finished function, with a type that has at least a results key
    let file_job = post_to_kiara_api("jobs/queue_job", &body).await?;
    let file_data_id = &file_job["results"]["file"];

    let create_table_body = json!({
        "operation_id": "create.table.from.file",
        "inputs": {
        "file": file_data_id,
        "first_row_is_header": true}
    });

    let table_job = post_to_kiara_api("jobs/queue_job", &create_table_body).await?;
    let edges = &dbg!(&table_job)["results"]["table"];
    let create_network_body = json!({
        "operation_id": "assemble.network_data",
        "inputs": {
            "edges": edges,
            "source_column": "Source",
            "target_column": "Target"
        }
    });
    let network_job = post_to_kiara_api("jobs/queue_job", &create_network_body).await?;
    let network_data = &network_job["results"]["network_data"].as_str().unwrap();

    get_from_kiara_api(&format!("data/value_info/{network_data}")).await
}

#[tauri::command]
async fn explain_operation(id: String) -> Result<serde_json::Value, String> {
    get_from_kiara_api(&format!("operations/{id}")).await
}

#[tauri::command]
async fn extract_largest_component(network_data_id: String) -> Result<serde_json::Value, String> {
    let body = json!({
      "operation_id": "network_data.extract_components",
      "operation_config": {},
      "inputs": {"network_data": network_data_id}
    });
    let components_job = post_to_kiara_api("jobs/queue_job", &body).await?;
    let components_network_data = &components_job["results"]["network_data"];
    let filter_body = json!({
      "operation_id": "network_data_filter.component",
      "operation_config": {},
      "inputs": {"value": components_network_data}
    });
    let filter_job = post_to_kiara_api("jobs/queue_job", &filter_body).await?;
    let filtered_network_data = &filter_job["results"]["value"].as_str().unwrap();
    get_from_kiara_api(&format!("data/value_info/{filtered_network_data}")).await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            query_kiara,
            list_operations,
            explain_operation,
            import_file,
            list_network_datas,
            extract_largest_component
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
