<script lang="ts">
	import type {NetworkMetadata} from "$lib"
	export let networkMetadata: NetworkMetadata;

	const direction = ['directed', 'undirected'] as const;
	const multi = [
		{ value: '_multi', label: 'multi' },
		{ value: '', label: 'not multi???' }
	] as const;

	let selected_multi = multi[0].value;
	let selected_direction = direction[0];
	$: key = `${selected_direction}${selected_multi}` as const;
	export let networkId: string;
</script>

<div class="px-4">
	<div class="px-4 sm:px-0 accent-purple-500 text-sm">
		<h2 class="text-base text-purple-600 font-semibold leading-7">Network metadata</h2>
		<span class="mt-1 max-w-2xl leading-6">Treat this network as:</span>
		{#each direction as value}
			<label>
				<input type="radio" bind:group={selected_direction} name="direction" {value} />
				{value}
			</label>{/each}
		{#each multi as value}
			<label>
				<input type="radio" bind:group={selected_multi} name="multi" value={value.value} />
				{value.label}
			</label>
		{/each}
	</div>
	<div class="mt-2 border-t border-gray-100">
		<dl class="divide-y divide-gray-100">
			<div>
				<dt>Number of nodes</dt>
				<dd>{networkMetadata.number_of_nodes}</dd>
			</div>
			<div>
				<dt>Number of self-loops</dt>
				<dd>{networkMetadata.number_of_self_loops}</dd>
			</div>
			<div>
				<dt>Number of edges</dt>
				<dd>{networkMetadata.properties_by_graph_type[key].number_of_edges}</dd>
			</div>
			{#if selected_multi.length}
				<div class="">
					<dt class="">Number of parallel edges</dt>
					<dd class="">{networkMetadata.properties_by_graph_type[key].parallel_edges}</dd>
				</div>
			{/if}
			<div>
				<dt>Network ID</dt>
				<dd>{networkId}</dd>
			</div>
		</dl>
	</div>
</div>

<style lang="postcss">
	dt {
		@apply text-sm font-medium leading-6 text-gray-900;
	}

	dd {
		@apply mt-1 text-sm leading-6 text-gray-800 sm:col-span-2 sm:mt-0;
	}

	dl > div {
		@apply px-2 py-2 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0;
	}
	label {
		@apply mr-2 inline-flex gap-1 items-center;
	}
</style>
