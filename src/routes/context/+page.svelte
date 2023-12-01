<script lang="ts">
	import SelectContext from '$lib/SelectContext.svelte';
	export let data;
</script>

<h1 class="pb-8">What research project are you working on?</h1>

<p>
	Use projects to separate out different areas of your research, where the data sets you are working
	with don't overlap.
</p>
<SelectContext all_contexts={data.all_contexts} current_context={data.context} />
