<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	type NetworkDataValueInfo = {
		value_id: string;
		properties: { 'metadata.network_data': object };
		// and some other stuff
	};

	export let data;

	let network_data = {};
	let network_data_id: string | undefined = undefined;

	let filepath = 'gh:/DHARPA-Project/kiara_plugin.dh_tagung_2023/main/docs/data/CKCC.csv';

	async function load_file() {
		// TODO typescript
		const response: NetworkDataValueInfo = await invoke('import_file', { filepath });
		network_data_id = response.value_id;
		network_data = response.properties['metadata.network_data'];
	}
</script>

<h1 class="text-4xl mb-8 text-purple-500 font-bold">Kiara demo</h1>
<p>
	TODO list any network datas you already have in your kiara, pick one of those to work on. This
	probably wants to use aliases
</p>
<label for="filepath">Load a file</label>
<input class="w-full" name="filepath" type="text" bind:value={filepath} />
<p>
	TODO let the user specify first_row_is_header, and source and target column names (with defaults?)
</p>
<p>TODO be able to load other kinds of files too</p>
<button class="bg-purple-300 p-2 rounded-md my-4" on:click={load_file}>Load</button>
{#if network_data_id}
	<p>
		You made a network_data thing with ID <code>{network_data_id}</code> Here's some data about it
	</p>
	<pre>{JSON.stringify(network_data, null, 2)}</pre>
{/if}

<p>TODO list calculations you can do on that network</p>
<p>TODO do a calculation</p>
<p>Here's a list of some operation ids</p>
<pre class="text-sm">{JSON.stringify(data.resp, null, 2)}</pre>
