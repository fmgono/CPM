import { preprocessMeltUI, sequence } from "@melt-ui/pp";
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
/** @type {import('@sveltejs/kit').Config}*/
const config = {
	preprocess: sequence([vitePreprocess(), preprocessMeltUI()]),
	kit: {
		adapter: adapter(),
		alias: {
			$actions: "./src/actions/*",
			$components: "./src/components/*",
			$lib: "./src/lib/*",
			$routes: "./src/routes/*",
			$types: "./src/types/*",
			$stores: "./src/stores/*",
			$styles: "./src/styles/*"
		},
		files: {
			assets: "./public"
		}
	}
};
export default config;
