<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import LoadingSpinner from '$lib/LoadingSpinner.svelte';
	import SelectContext from '$lib/SelectContext.svelte';

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

<h1 class="py-8">Welcome to doing network analysis with kiara!</h1>
<p>What context are you working in?</p>
<SelectContext all_contexts={data.all_contexts} current_context={data.context} />
