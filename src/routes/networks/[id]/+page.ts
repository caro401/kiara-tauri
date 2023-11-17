/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
	const { invoke } = await import('@tauri-apps/api');
	const nodes = await invoke('get_network_nodes_table', { networkId: params.id });
	const metadata = await invoke('get_network_metadata', { networkId: params.id });

	const network_datas = await invoke('list_network_datas');
	const network_alias = network_datas.filter((v) => v[1] == params.id)[0][0];

	return { nodes, networkId: params.id, network_alias, metadata };
}
