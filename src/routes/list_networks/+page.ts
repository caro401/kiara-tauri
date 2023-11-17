export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	const network_datas = await invoke('list_network_datas');

	return { network_datas };
}
