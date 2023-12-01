import type { NetworkMetadata, NetworkNode}from "$lib";

/** @type {import('./$types').PageLoad} */
export async function load({params}) {
    const {invoke} = await import('@tauri-apps/api');
    const nodes: NetworkNode[] = JSON.parse(await invoke('get_network_nodes_table', {networkId: params.id}));
    const metadata: NetworkMetadata = JSON.parse(await invoke('get_network_metadata', {networkId: params.id}));

    const network_datas: string[][] = await invoke('list_network_datas');

    const network_alias: string = network_datas.filter((v) => v[1] == params.id)[0][0];

    return {nodes, networkId: params.id, network_alias, metadata};
}
