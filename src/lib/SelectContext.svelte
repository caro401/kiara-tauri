<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import toast from 'svelte-french-toast';
	import { goto, invalidateAll } from '$app/navigation';
	import { validateAlias } from '$lib/index';

	export let all_contexts: string[];
	export let current_context: string;
	let selected = current_context;
	let newName = '';

	let error: string | undefined;

	async function load_context() {
		await invoke('load_context', { contextName: selected });
		toast.success(`Loaded project ${selected}`);
		await invalidateAll();
		goto('/');
	}

	async function create_context() {
		if (!validateAlias(newName)) {
			error =
				"That name contains characters that aren't supported, please only use letters, numbers, - or _";
			return;
		}
		await invoke('load_context', { contextName: newName });
		toast.success(`Created project ${selected}`);
		error = undefined;
		await invalidateAll();
		goto('/');
	}
</script>

<h2>Load an existing project</h2>
<form on:submit|preventDefault={load_context}>
	<select bind:value={selected}>
		{#each all_contexts as c}
			<option>{c}</option>
		{/each}
	</select>
	<button>Load project</button>
</form>

<h2>Create a new project</h2>
<form on:submit|preventDefault={create_context}>
	<label for="newContextName" class="block">Project name</label>
	{#if error}
		<p class="error">{error}</p>
	{/if}

	<input name="newContextName" required bind:value={newName} type="text" />
	<button>Create project</button>
	<span class="text-sm block">(letters, numbers, - and _ only, no spaces)</span>
</form>
