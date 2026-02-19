<script lang="ts">
	import type { Snippet } from 'svelte';
	let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
	<style>
		html,
		body {
			background: transparent !important;
			overflow: hidden;
			pointer-events: none;
			cursor: none;
			user-select: none;
		}
	</style>
</svelte:head>

{@render children()}
