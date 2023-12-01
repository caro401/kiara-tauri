export const prerender = false;
export const ssr = false;

/** @type {import('./$types').LayoutLoad} */
export async function load() {
	const { invoke } = await import('@tauri-apps/api');
	const context: string = await invoke('get_current_context_name');

	return { context };
}
