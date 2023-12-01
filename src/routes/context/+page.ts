/** @type {import('./$types').PageLoad} */
export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	const all_contexts: string[] = await invoke('list_context_names');
	return { all_contexts };
}
