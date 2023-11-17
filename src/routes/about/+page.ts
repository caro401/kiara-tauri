export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	const python_version = await invoke('get_python_version');
	const plugins = await invoke('list_plugin_versions');

	return { python_version, plugins };
}
