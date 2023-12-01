/** @type {import('./$types').PageLoad} */
export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	const existing_aliases: string[] = await invoke('list_aliases');

	return { existing_aliases };
}
