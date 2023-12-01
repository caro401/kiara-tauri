use crate::pyo3_helpers::call_kiara_function;

#[tauri::command]
pub async fn get_current_context_name() -> Result<String, String> {
    call_kiara_function("response = kiara.get_current_context_name()")
}

#[tauri::command]
pub async fn load_context(context_name: String) -> Result<String, String> {
    call_kiara_function(&format!(
        "kiara.set_active_context('{context_name}', create=True)\nresponse =  'success!'"
    ))
}

#[tauri::command]
pub async fn list_context_names() -> Result<Vec<String>, String> {
    call_kiara_function("response = kiara.list_context_names()")
}
