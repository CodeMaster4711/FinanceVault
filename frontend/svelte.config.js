import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			fallback: 'index.html'
		}),
		alias: {
			$components: 'src/components',
			$stores: 'src/stores',
			$lib: 'src/lib',
			$utils: 'src/utils'
		}
	}
};

export default config;
