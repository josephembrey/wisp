<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { log } from '$lib/log';
	import { onMount } from 'svelte';

	let { children } = $props();

	onMount(() => {
		log.info(`[layout] mounted, url=${window.location.pathname}`);
	});
</script>

<ModeWatcher defaultMode="dark" />
<Toaster position="bottom-center" />
<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
