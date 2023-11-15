<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import LoadingSpinner from '$lib/LoadingSpinner.svelte';

	type NetworkDataValueInfo = {
		value_id: string;
		properties: { 'metadata.network_data': object };
		// and some other stuff
	};

	type OperationData = {
		type_name: string;
		documentation: { description: string; doc: string | null };
	};

	export let data;
	$: operationsData = data.operations as Record<string, OperationData>;
	let network_data = {};
	let networkDataId: string | undefined = undefined;
	let largestComponent: object | undefined = undefined;
	let loadingComponent = false;
	let loadingFile = false;
	let filepath = 'gh:/DHARPA-Project/kiara_plugin.dh_tagung_2023/main/docs/data/CKCC.csv';

	async function load_file() {
		loadingFile = true;
		const response: NetworkDataValueInfo = await invoke('import_file', { filepath });
		networkDataId = response.value_id;
		network_data = response.properties['metadata.network_data'];
		loadingFile = false;
	}

	async function extract_largest_component() {
		loadingComponent = true;
		const response: NetworkDataValueInfo = await invoke('extract_largest_component', {
			networkDataId
		});
		largestComponent = response.properties['metadata.network_data'];
		loadingComponent = false;
	}

	// const demoNetworkId = '88ed2bf3-cd76-4ad0-8236-6198066b39a2';
</script>

<h1>Name of currently active context here</h1>
<p>Button to switch contexts? aka load a different project?</p>

<!--<p>-->
<!--	TODO list any network datas you already have in your kiara, pick one of those to work on. This-->
<!--	probably wants to use aliases-->
<!--</p>-->
<h2>Here's the network datas you've already loaded</h2>

<pre>{JSON.stringify(data.operations, null, 2)}</pre>
<h2>Load some more data?</h2>
<label for="filepath">Load a file</label>
<input class="w-full" name="filepath" type="text" bind:value={filepath} />
<!--<p>-->
<!--	TODO let the user specify first_row_is_header, and source and target column names (with defaults?)-->
<!--</p>-->
<!--<p>TODO be able to load other kinds of files too</p>-->
<button on:click={load_file}
	>{#if loadingFile}<LoadingSpinner />{:else}Load{/if}</button
>
{#if networkDataId}
	<p>
		You made a network_data thing with ID <code>{networkDataId}</code> Here's some data about it
	</p>
	<pre>{JSON.stringify(network_data, null, 2)}</pre>
{/if}
{#if networkDataId}
	<h2>Do something with your network</h2>
	<button on:click={extract_largest_component}
		>{#if loadingComponent}<LoadingSpinner />{:else}Extract largest component{/if}</button
	>
	{#if largestComponent}
		<pre>{JSON.stringify(largestComponent, null, 2)}</pre>
	{/if}
	<p class="my-4">There's some other operations too, but you can't do them yet</p>
	<ul class="my-8">
		{#each Object.keys(operationsData) as o}
			{@const operation = operationsData[o]}
			<li>
				{operation.type_name}: {operation.documentation
					.description}{#if operation.documentation.doc}<br />{operation.documentation.doc}{/if}
			</li>
		{/each}
	</ul>
{/if}
<!--<p>TODO list calculations you can do on that network</p>-->
<!--<p>TODO do a calculation</p>-->
