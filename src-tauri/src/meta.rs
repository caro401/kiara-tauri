use crate::pyo3_helpers::call_kiara_function;

#[tauri::command]
pub async fn list_plugin_versions() -> Result<Vec<String>, String> {
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
