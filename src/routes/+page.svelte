<script lang="ts">
	export let data;
</script>

<header class="flex justify-between items-center">
	<h1>Networks in project <span class="italic">{data.context}</span></h1>
	<a class="button" href="/context">Switch project</a>
</header>
<a class="button" href="/import_file">Import a new network</a>

{#if !data.network_datas.length}
	<p>No networks yet in project {data.context}.</p>
{:else}
	<ul>
		{#each data.network_datas as n}
			<li><a href={`/networks/${n[1]}`}>{n[0]}</a></li>{/each}
	</ul>
{/if}
