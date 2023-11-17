<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import LoadingSpinner from '$lib/LoadingSpinner.svelte';
	import { goto } from '$app/navigation';
	export let data;
	let filepath =
		'/Volumes/source/github.com/DHARPA-Project/kiara_plugin.dh_tagung_2023/docs/data/CKCC.csv';
	let loading = false;
	let edges_table = undefined;
	let edgesTableId = undefined;
	let alias = '';
	let loading_network = false;
	async function load_file() {
		loading = true;
		const response = await invoke('import_file', { filepath });
		edges_table = response[1];
		edgesTableId = response[0];

		loading = false;
	}
	import Grid from 'gridjs-svelte';
	import toast from 'svelte-french-toast';

	async function create_network() {
		loading_network = true;
		// const alias = (Math.random() + 1).toString(36).substring(7);
		const response = await invoke('create_network_from_edges_table', {
			edgesTableId,
			alias: alias
		});
		toast.success(`Made a network with name ${alias} and id ${response}`);
		goto('/list_networks');
	}
</script>

<h1>Create a network in {data.context}</h1>

<h2>Create from a CSV file</h2>
<p>
	At the moment, we're assuming you have your graph edges in a spreadsheet (csv file), with at least
	2 columns. The columns are called <code>Source</code> and <code>Target</code>, and each row
	represents an edge in your graph
</p>
<p>There will be another tab here to import from Tropy's JSON-LD export at some point</p>
<label class="block mt-8" for="filepath">Load a file</label>
<input class="w-full" name="filepath" type="text" bind:value={filepath} />
<button on:click={load_file}
	>{#if loading}<LoadingSpinner />{:else}Load{/if}</button
>

{#if edges_table}
	<h2>Here's what the edges in your graph will be if you import that file, does it look right?</h2>
	<p>If not, go back and fix things in your spreadsheet, then reload the file here</p>
	<p>It has {edges_table.length} rows</p>
	<Grid
		columns={['Source', 'Target']}
		sort={true}
		search={true}
		pagination={{ limit: 50 }}
		data={edges_table}
	/>
	<div>
		<label for="alias">Give your network a useful but unique name</label><input
			name="alias"
			type="text"
			bind:value={alias}
		/>

		<button on:click={create_network}>
			{#if loading_network}<LoadingSpinner />{:else}Create network{/if}</button
		>
	</div>
{/if}

<style global>
	@import 'https://cdn.jsdelivr.net/npm/gridjs/dist/theme/mermaid.min.css';
</style>
