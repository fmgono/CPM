<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import { File } from "lucide-svelte";
	import { Button } from "$lib/components/ui/button";
	import { Card, CardDescription, CardHeader, CardTitle } from "$lib/components/ui/card";
	import { Checkbox } from "$lib/components/ui/checkbox";
	import {
		Table,
		TableBody,
		TableCell,
		TableHead,
		TableHeader,
		TableRow
	} from "$lib/components/ui/table";
	import { onMount } from "svelte";

	let projects = [];
	onMount(async () => {
		// projects = await invoke("list_folders")
		projects = await invoke("list_projects");
		// console.log('listProjects', listProjects)
	});
</script>

<svelte:head>
	<title>SvelteKit Starter Template</title>
	<meta
		name="description"
		content="A starter template for SvelteKit w/ TypeScript, TailwindCSS pre-configured with ESLint, Prettier and Husky git hooks."
	/>
</svelte:head>

<main class="m-8 flex min-h-screen w-full flex-col">
	<!-- <section>
    <h2 class="text-2xl font-semibold mb-4">Explore By Format</h2>
    <div class="grid grid-cols-4 gap-4 mb-8">
      <Card class="bg-[#fee2e2]">
        <CardHeader>
          <File />
          <CardTitle>Audio Files</CardTitle>
          <CardDescription>Size: 15 GB</CardDescription>
        </CardHeader>
      </Card>
      <Card class="bg-[#e0e7ff]">
        <CardHeader>
          <File />
          <CardTitle>Video Files</CardTitle>
          <CardDescription>Size: 48 GB</CardDescription>
        </CardHeader>
      </Card>
      <Card class="bg-[#e0e7ff]">
        <CardHeader>
          <File />
          <CardTitle>PDF Files</CardTitle>
          <CardDescription>Size: 10 GB</CardDescription>
        </CardHeader>
      </Card>
      <Card class="bg-[#e0e7ff]">
        <CardHeader>
          <File />
          <CardTitle>Zip Files</CardTitle>
          <CardDescription>Size: 60 GB</CardDescription>
        </CardHeader>
      </Card>
    </div>
  </section> -->
	<h3 class="mb-4 text-xl font-semibold">All Projects</h3>
	<Table>
		<TableHeader>
			<TableRow>
				<TableHead class="w-52">Name</TableHead>
				<TableHead>Size</TableHead>
				<TableHead>Last Modified</TableHead>
				<TableHead>Last Opened</TableHead>
			</TableRow>
		</TableHeader>
		<TableBody>
			{#each projects as project}
				<TableRow>
					<TableCell class="flex w-52 items-center gap-2 font-medium ">
						<File />
						<p class="truncate">
							{project.name}
						</p>
					</TableCell>
					<TableCell>{project.size}</TableCell>
					<TableCell>10th Feb 2024</TableCell>
					<TableCell>17th Feb 2024</TableCell>
				</TableRow>
			{/each}
		</TableBody>
	</Table>

	<!-- <pre>{JSON.stringify(projects)}</pre> -->
</main>
