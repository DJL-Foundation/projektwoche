import devtoolsJson from 'vite-plugin-devtools-json';
import { vercelToolbar } from '@vercel/toolbar/plugins/vite';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), vercelToolbar(), devtoolsJson()]
});
