<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import LoadingSpinner from '$lib/LoadingSpinner.svelte';
	import { goto } from '$app/navigation';
	import toast from 'svelte-french-toast';
	import FilePicker from '$lib/FilePicker.svelte';
	import TablePreview from '$lib/TablePreview.svelte';
	import { validateAlias } from '$lib';

	type ImportedTable = {
		tableData: string[][];
		tableId: string;
	};

	export let data;

	let alias = '';
	let loading_network = false;
	let loadFileResponse: ImportedTable;
	$: edgesTable = loadFileResponse ? loadFileResponse.tableData : undefined;
	$: edgesTableId = loadFileResponse ? loadFileResponse.tableId : undefined;
	let error: string | undefined = undefined;

	function validateNewAlias(a: string) : boolean  {
		if (!validateAlias(a)) {
			error = "Name can only contain letters, numbers, - or _, and can't be empty";
			return false;
		}
		if (data.existing_aliases.includes(a)) {
			error = "You've already used that name for a different network, please pick a unique name";
			return false;
		}
		error = '';
		return true;
	}

	async function create_network() {
		if (!validateNewAlias(alias)) {
			return;
		}
		loading_network = true;
		const response = await invoke('create_network_from_edges_table', {
			edgesTableId,
			alias: alias
		});
		error = undefined;
		toast.success(`Made a network with name ${alias} and id ${response}`);
		goto('/list_networks');
	}
</script>

<h1>Create a network in project {data.context}</h1>

<p>
	At the moment, we're assuming you have your graph edges in a spreadsheet (csv file), with at least
	2 columns. The columns are called <code>Source</code> and <code>Target</code>, and each row
	represents an edge in your graph.
</p>
<p class="py-4">
	There will be another tab here to import from Tropy's JSON-LD export at some point
</p>
<label for="alias" class="block mt-2">Give your network a useful but unique name</label>
<input
	name="alias"
	on:input={(e) => validateNewAlias(e.target?.value)}
	type="text"
	bind:value={alias}
/>
{#if error}<p class="error">{error}</p>{/if}
<FilePicker bind:loadFileResponse />

{#if edgesTable}
	<h2>Here's what the edges in your graph will be if you import that file, does it look right?</h2>
	<p>If not, go back and fix things in your spreadsheet, then reload the file here</p>
	<p>It has {edgesTable.length} rows</p>

	<button class="w-full text-center" on:click={create_network}>
		{#if loading_network}
			<LoadingSpinner />
		{:else}
			Create network
		{/if}
	</button>

	<TablePreview columns={['Source', 'Target']} data={edgesTable} />
{/if}
