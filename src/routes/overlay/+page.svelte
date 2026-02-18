<script lang="ts">
	import { onMount } from 'svelte';
	import { onStatusChanged, type Status } from '$lib/tauri';

	let status = $state<Status>('idle');

	onMount(() => {
		const unlisten = onStatusChanged((s) => {
			status = s;
		});
		return () => {
			unlisten.then((fn) => fn());
		};
	});
</script>

<svelte:head>
	<style>
		html,
		body {
			background: transparent !important;
			overflow: hidden;
			pointer-events: none;
		}
	</style>
</svelte:head>

{#if status !== 'idle'}
	<div class="flex h-screen w-screen items-start justify-center">
		<div
			class="mt-1 flex items-center gap-2 rounded-full border border-white/10 bg-black/80 px-3 py-1.5 shadow-lg backdrop-blur-sm"
		>
			{#if status === 'recording'}
				<span class="relative flex h-2.5 w-2.5">
					<span
						class="absolute inline-flex h-full w-full animate-ping rounded-full bg-red-400 opacity-75"
					></span>
					<span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-red-500"></span>
				</span>
				<span class="text-xs font-medium text-white">Recording</span>
			{:else if status === 'processing'}
				<span class="relative flex h-2.5 w-2.5">
					<span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-amber-500"></span>
				</span>
				<svg
					class="h-3 w-3 animate-spin text-amber-400"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
					></path>
				</svg>
				<span class="text-xs font-medium text-white">Processing</span>
			{:else if status === 'loading'}
				<span class="relative flex h-2.5 w-2.5">
					<span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-blue-500"></span>
				</span>
				<svg
					class="h-3 w-3 animate-spin text-blue-400"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
					></path>
				</svg>
				<span class="text-xs font-medium text-white">Loading</span>
			{/if}
		</div>
	</div>
{/if}
