<script lang="ts">
	import Grid from 'gridjs-svelte';
	import NetworkMetadata from '$lib/NetworkMetadata.svelte';

	export let data;
</script>

<h1>Network {data.network_alias}</h1>
<NetworkMetadata networkMetadata={JSON.parse(data.metadata)} networkId={data.networkId} />
<h2>Nodes</h2>
<Grid
	search={true}
	columns={[
		{ id: '_node_id', name: 'ID' },
		{ id: 'id', name: 'Label' }
	]}
	sort={true}
	pagination={{ limit: 50 }}
	data={JSON.parse(data.nodes)}
/>
