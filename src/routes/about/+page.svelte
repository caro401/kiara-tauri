<script lang="ts">
	export let data;
</script>

<h1>About</h1>

<p>using {data.python_version}</p>

<p>With these plugins</p>

<ul class="list-disc px-8">
	{#each data.plugins as p}
		<li>{p}</li>
	{/each}
</ul>
