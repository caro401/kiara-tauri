/** @type {import('./$types').PageLoad} */
export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	const python_version: string = await invoke('get_python_version');
	const plugins: string[] = await invoke('list_plugin_versions');

	return { python_version, plugins };
}
