<script lang="ts">
	import Greet from "$components/greet.svelte";
</script>

<svelte:head>
	<title>SvelteKit Starter Template</title>
	<meta
		name="description"
		content="A starter template for SvelteKit w/ TypeScript, TailwindCSS pre-configured with ESLint, Prettier and Husky git hooks."
	/>
</svelte:head>

<main class="min-h-screen w-full">
	<Greet />
</main>
