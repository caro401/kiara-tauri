// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
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

fn call_kiara_function<D>(python_code: &str) -> Result<D, String>
where
    for<'p> D: FromPyObject<'p>,
{
    Ok(Python::with_gil(|py| {
        let kiara_import = py.import("kiara.api")?;
        let kiara_api = kiara_import.getattr("KiaraAPI")?;
        let kiara = kiara_api.call_method0("instance")?;
        let locals = [("kiara", kiara)].into_py_dict(py);
        py.run(python_code, None, Some(&locals))?;
        let response = locals.get_item("response").unwrap().unwrap();
        response.extract()
    })
    .unwrap())

}
#[tauri::command]
async fn list_operations() -> Result<Vec<String>, String> {
    call_kiara_function("response = kiara.list_operation_ids()")
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
async fn explain_operation(id: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "kiara.retrieve_operation_info('{id}').create_html()"
    ))
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
