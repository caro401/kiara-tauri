use crate::pyo3_helpers::call_kiara_function;
use pyo3::FromPyObject;
use serde::Serialize;
use serde_json::json;

// this guy could really do with being an object {tableId: string, tableData: arrow table?}
#[derive(FromPyObject, Serialize)]
#[serde(untagged)]
pub enum ImportTable {
    // Assuming for now that your edges table either has string nodes or number nodes
    StringTable(Vec<(String, String)>),
    NumberTable(Vec<(i32, i32)>),
}

#[tauri::command]
pub async fn import_file(filepath: String) -> Result<serde_json::Value, String> {
    let response: (String, ImportTable) = call_kiara_function(&format!("
file_results = kiara.run_job('import.local.file', inputs={{'path': '{filepath}'}})
table_results = kiara.run_job('create.table.from.file', inputs={{'file': file_results['file'], 'first_row_is_header': True}})
edges_table = table_results['table'].data.arrow_table.to_pylist()
edges_table_id = str(table_results['table'].value_id)
response = (edges_table_id, [(v['Source'], v['Target']) for v in edges_table])
"))?;
    Ok(json!({
        "tableId": response.0,
        "tableData": response.1
    }))
}

#[tauri::command]
pub async fn create_network_from_edges_table(
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
