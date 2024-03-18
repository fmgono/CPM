<script lang="ts">
	import Button from "$lib/components/ui/button/button.svelte";
	import { invoke } from "@tauri-apps/api/tauri";

	let name = "";
	let greetMsg = "";

	async function greet() {
		greetMsg = await invoke("greet", { name });
	}
</script>

<div>
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<Button on:click={greet}>Greet</Button>
	<p>{greetMsg}</p>
</div>
