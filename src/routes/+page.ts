/** @type {import('./$types').PageLoad} */
export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	const resp = await invoke('list_operation_ids');

	return { resp };
}
