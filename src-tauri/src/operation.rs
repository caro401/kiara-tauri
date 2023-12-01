use crate::pyo3_helpers::call_kiara_function;

#[tauri::command]
pub async fn list_operations() -> Result<Vec<String>, String> {
    call_kiara_function("response = kiara.list_operation_ids()")
}

#[tauri::command]
pub async fn explain_operation(id: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "kiara.retrieve_operation_info('{id}').create_html()"
    ))
}
