<script lang="ts">
	import Grid from 'gridjs-svelte';
	type Column = { id: string; name: string };

	export let data: string[][];
	export let columns: string[] | Column[];
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
