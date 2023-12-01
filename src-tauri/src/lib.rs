mod context;
mod data;
mod meta;
mod operation;
mod pyo3_helpers;
mod run_job;

use context::*;
use data::*;
use meta::*;
use operation::*;
use run_job::*;

use pyo3::Python;

#[tauri::command]
async fn get_python_version() -> String {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        sys.getattr("version")?.extract()
    })
    .unwrap()
}

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_python_version,
            list_plugin_versions,
            get_current_context_name,
            list_context_names,
            load_context,
            list_operations,
            explain_operation,
            list_aliases,
            import_file,
            list_network_datas,
            create_network_from_edges_table,
            get_network_nodes_table,
            get_network_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
