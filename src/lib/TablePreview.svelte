<script lang="ts">
	import Grid from 'gridjs-svelte';

	export let data: string[][];
	$: console.log(data);
	export let columns: string[];
</script>

<Grid
	{columns}
	sort={true}
	search={true}
	pagination={{ limit: 50 }}
	{data}
	language={{
		search: {
			placeholder: '🔍 Search...'
		}
	}}
/>
