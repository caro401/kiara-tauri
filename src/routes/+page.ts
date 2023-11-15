/** @type {import('./$types').PageLoad} */
export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	// const existing_network_data = await invoke('list_network_datas');
	const operations = await invoke('list_operations');

	return { operations };
}
