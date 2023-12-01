// place files you want to import through the `$lib` alias in this folder.

// Aliases and context names can only be alphanumeric, - or _ for now
export function validateAlias(alias: string): boolean {
	return alias.match(/^[\w\-_]{3,}$/) != null;
}


export type NetworkNode = { _node_id: number, _label: string, _count_edges: number, _count_edges_multi: number, _in_edges: number, _in_edges_multi: number, _out_edges: number, _out_edges_multi: number, id: string }
export type GraphTypeData = { number_of_edges: number, parallel_edges: number }
export type NetworkMetadata = { number_of_nodes: number, properties_by_graph_type: { directed: GraphTypeData, directed_multi: GraphTypeData, undirected: GraphTypeData, undirected_multi: GraphTypeData}, number_of_self_loops: number }
