<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import LoadingSpinner from '$lib/LoadingSpinner.svelte';

	let selectedFileName: string | string[] | null = '';
	export let loadFileResponse;
	let loading = false;

	async function load_file() {
		loading = true;
		loadFileResponse = await invoke('import_file', { filepath: selectedFileName });
		console.log(loadFileResponse);
		loading = false;
	}

	async function pickFile() {
		selectedFileName = await open({
			multiple: false,
			filters: [
				{
					name: 'Spreadsheet',
					extensions: ['csv']
				}
			]
		});
	}
</script>

<div class="flex gap-x-4 items-center">
	<button class="flex-shrink-0" on:click={pickFile}>Pick a file</button>
	<p class="text-sm">{selectedFileName}</p>
</div>
{#if selectedFileName && selectedFileName.length > 0}
	<button on:click={load_file}>
		{#if loading}
			<LoadingSpinner />
		{:else}
			Load file
		{/if}
	</button>
{/if}
