import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

// Suppress unused Channel import in auto-generated bindings (see also eslint.config.js)
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const onwarn = (warning: any, defaultHandler: any) => {
	if (warning.code === 'UNUSED_EXTERNAL_IMPORT' && warning.exporter === '@tauri-apps/api/core')
		return;
	defaultHandler(warning);
};

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	build: {
		rollupOptions: { onwarn }
	},
	environments: {
		ssr: {
			build: {
				rollupOptions: { onwarn }
			}
		}
	}
});
