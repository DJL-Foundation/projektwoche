import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter from '@sveltejs/adapter-auto';
import { withMicrofrontends } from '@vercel/microfrontends/experimental/sveltekit';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter()
	}
};

const microfrontendConfig = withMicrofrontends(config);

export default microfrontendConfig;
