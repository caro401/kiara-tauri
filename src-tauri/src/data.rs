use crate::pyo3_helpers::call_kiara_function;
use pyo3::FromPyObject;
use serde::Serialize;

#[tauri::command]
pub async fn list_network_datas() -> Result<Vec<(String, String)>, String> {
    call_kiara_function(
        "\
data = kiara.list_aliases(data_types=['network_data']).get_all_value_ids()
response = [(k, str(v)) for k, v in data.items()]",
    )
}

#[tauri::command]
pub async fn get_network_nodes_table(network_id: String) -> Result<String, String> {
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
pub async fn get_network_metadata(network_id: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "
import json
network = kiara.get_value('{network_id}')
meta = network.get_property_data('metadata.network_data')
response = json.dumps(meta.dict())"
    ))
}

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

#[tauri::command]
pub async fn list_aliases() -> Result<Vec<String>, String> {
    call_kiara_function("response = kiara.list_aliases()")
}
