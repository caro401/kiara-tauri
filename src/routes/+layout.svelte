<script>
	import '../app.postcss';
	import { Toaster } from 'svelte-french-toast';
</script>

<header class="p-2">
	<nav><a class="underline" href="/">Home</a> <a class="underline" href="/about">About</a></nav>
</header>
<main class="p-4">
	<slot />
</main>
<Toaster />
