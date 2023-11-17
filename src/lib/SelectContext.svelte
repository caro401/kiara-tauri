<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import toast from 'svelte-french-toast';
	import { goto, invalidateAll } from '$app/navigation';
	export let all_contexts: string[];
	export let current_context: string;
	let selected = current_context;
	async function load_context() {
		const response = await invoke('load_context', { contextName: selected });
		console.log(response);
		toast.success(`Loaded context ${selected}`);
		await invalidateAll();
		goto('/list_networks');
	}
</script>

<select bind:value={selected}>
	{#each all_contexts as c}
		<option>{c}</option>
	{/each}
</select>
<button on:click={load_context}>Go!</button>

<p>todo be able to create another context, doable but i didn't do it yet</p>
