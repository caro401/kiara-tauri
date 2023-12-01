<script lang="ts">
	import NetworkMetadata from '$lib/NetworkMetadata.svelte';
	import TablePreview from '$lib/TablePreview.svelte';

	export let data;
</script>

<h1>Network {data.network_alias}</h1>
<NetworkMetadata networkMetadata={data.metadata} networkId={data.networkId} />
<h2>Nodes</h2>
<TablePreview
	data={data.nodes}
	columns={[
		{ id: '_node_id', name: 'ID' },
		{ id: 'id', name: 'Label' }
	]}
/>
<h2>Edges</h2>

<TablePreview
	data={data.edges}
	columns={[
		{ id: '_source', name: 'Source' },
		{ id: '_target', name: 'Target' }
	]}
/>
