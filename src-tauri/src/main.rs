// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use serde::{Deserialize, Serialize};
// use serde_json::json;
//
// async fn get_from_kiara_api(path: &str) -> Result<serde_json::Value, String> {
//     let client = reqwest::Client::new();
//     client
//         .get(format!("http://localhost:8080/{path}"))
//         .send()
//         .await
//         .map_err(|e| {
//             format!(
//                 "Failed to connect to Kiara API: {e} Did you forget to start your Kiara server?"
//             )
//         })?
//         .error_for_status()
//         .map_err(|e| format!("HTTP error in response from kiara: {e}"))?
//         .json()
//         .await
//         .map_err(|e| format!("Failed to decode response from kiara: {e}"))
// }
//
// async fn post_to_kiara_api(
//     path: &str,
//     body: &serde_json::Value,
// ) -> Result<serde_json::Value, String> {
//     let client = reqwest::Client::new();
//     client
//         .post(format!("http://localhost:8080/{path}"))
//         .json(body)
//         .send()
//         .await
//         .map_err(|e| {
//             format!(
//                 "Failed to connect to Kiara API: {e} Did you forget to start your Kiara server?"
//             )
//         })?
//         .error_for_status()
//         .map_err(|e| format!("HTTP error in response from kiara: {e}"))?
//         .json()
//         .await
//         .map_err(|e| format!("Failed to decode response from kiara: {e}"))
// }

#[derive(FromPyObject, Serialize)]
struct NodesTableData {
    // {'_node_id': 0, '_label': 'Aa, Anthony Willemsz., 1582-1638', '_count_edges': 1, '_count_edges_multi': 1, '_in_edges': 1, '_in_edges_multi': 1, '_out_edges': 0, '_out_edges_multi': 0, 'id': 'Aa, Anthony Willemsz., 1582-1638'},
    _node_id: i32,
    _label: String,
    _count_edges: i32,
    _count_edges_multi_: i32,
    _in_edges: i32,
    _in_edges_multi: i32,
    _out_edges: i32,
    _out_edges_multi: i32,
    id: String,
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
async fn list_plugin_versions() -> Result<Vec<String>, String> {
    call_kiara_function(
        "\
from kiara.registries.environment import EnvironmentRegistry
registry = EnvironmentRegistry.instance()
python_env = registry.environments['python']
kiara_version = None
plugins = []
for pkg in python_env.packages:
    if pkg.name.startswith('kiara'):
        plugins.append( f'{pkg.name}: {pkg.version}')
response  = plugins
",
    )
}

#[tauri::command]
async fn list_operations() -> Result<Vec<String>, String> {
    call_kiara_function("response = kiara.list_operation_ids()")
}

#[tauri::command]
async fn get_current_context_name() -> Result<String, String> {
    call_kiara_function("response = kiara.get_current_context_name()")
}

#[tauri::command]
async fn load_context(context_name: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "kiara.set_active_context('{context_name}')\nresponse =  'success!'"
    ))
}

#[tauri::command]
async fn list_context_names() -> Result<Vec<String>, String> {
    call_kiara_function("response = kiara.list_context_names()")
}
#[tauri::command]
async fn list_network_datas() -> Result<Vec<(String, String)>, String> {
    call_kiara_function(
        "\
data = kiara.list_aliases(data_types=['network_data']).get_all_value_ids()
response = [(k, str(v)) for k, v in data.items()]",
    )
}

#[tauri::command]
async fn import_file(filepath: String) -> Result<(String, Vec<(String, String)>), String> {
    call_kiara_function(&format!("
file_results = kiara.run_job('import.local.file', inputs={{'path': '{filepath}'}})
table_results = kiara.run_job('create.table.from.file', inputs={{'file': file_results['file'], 'first_row_is_header': True}})
edges_table = table_results['table'].data.arrow_table.to_pylist()
edges_table_id = str(table_results['table'].value_id)
response = (edges_table_id, [(v['Source'], v['Target']) for v in edges_table])
"))
}

#[tauri::command]
async fn create_network_from_edges_table(
    edges_table_id: String,
    alias: String,
) -> Result<String, String> {
    call_kiara_function(&format!("
network_results = kiara.run_job('assemble.network_data', inputs={{'edges': '{edges_table_id}', 'source_column': 'Source', 'target_column': 'Target'}})
network_id = network_results['network_data'].value_id
kiara.store_value(network_id, '{alias}')
response = str(network_id)
"))
}

#[tauri::command]
async fn explain_operation(id: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "kiara.retrieve_operation_info('{id}').create_html()"
    ))
}

#[tauri::command]
async fn get_network_nodes_table(network_id: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "
import json
network = kiara.get_value('{network_id}').data
edges_table = network.get_table('nodes')
edges_table_data = edges_table.arrow_table.to_pylist()
response = json.dumps(edges_table_data)"
    ))
}

#[tauri::command]
async fn get_network_metadata(network_id: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "
import json
network = kiara.get_value('{network_id}')
meta = network.get_property_data('metadata.network_data')
response = json.dumps(meta.dict())"
    ))
}

//
// #[tauri::command]
// async fn extract_largest_component(network_data_id: String) -> Result<serde_json::Value, String> {
//     let body = json!({
//       "operation_id": "network_data.extract_components",
//       "operation_config": {},
//       "inputs": {"network_data": network_data_id}
//     });
//     let components_job = post_to_kiara_api("jobs/queue_job", &body).await?;
//     let components_network_data = &components_job["results"]["network_data"];
//     let filter_body = json!({
//       "operation_id": "network_data_filter.component",
//       "operation_config": {},
//       "inputs": {"value": components_network_data}
//     });
//     let filter_job = post_to_kiara_api("jobs/queue_job", &filter_body).await?;
//     let filtered_network_data = &filter_job["results"]["value"].as_str().unwrap();
//     get_from_kiara_api(&format!("data/value_info/{filtered_network_data}")).await
// }

#[tauri::command]
async fn get_python_version() -> String {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        sys.getattr("version")?.extract()
    })
    .unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_python_version,
            list_plugin_versions,
            get_current_context_name,
            list_context_names,
            load_context,
            list_operations,
            explain_operation,
            import_file,
            list_network_datas,
            create_network_from_edges_table,
            get_network_nodes_table,
            get_network_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
