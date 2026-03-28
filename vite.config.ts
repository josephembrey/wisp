import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	build: {
		rollupOptions: {
			onwarn(warning, defaultHandler) {
				// Suppress unused import in auto-generated bindings (see also eslint.config.js)
				if (
					warning.code === 'UNUSED_EXTERNAL_IMPORT' &&
					warning.exporter === '@tauri-apps/api/core'
				)
					return;
				defaultHandler(warning);
			}
		}
	}
});
