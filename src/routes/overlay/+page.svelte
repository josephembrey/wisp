<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getSettings,
		onStatusChanged,
		onOverlayFlash,
		onSettingsChanged,
		type Status
	} from '$lib/tauri';

	let status = $state<Status>('idle');
	let flash = $state('');
	let position = $state('top-center');
	let size = $state('medium');
	let flashTimeout: ReturnType<typeof setTimeout> | undefined;

	const justify = $derived(
		position.includes('left')
			? 'justify-start'
			: position.includes('right')
				? 'justify-end'
				: 'justify-center'
	);

	const align = $derived(position.includes('bottom') ? 'items-end' : 'items-start');

	const scale = $derived(size === 'small' ? 'scale-75' : size === 'large' ? 'scale-150' : '');

	onMount(() => {
		getSettings().then((s) => {
			position = s.overlay_position;
			size = s.overlay_size;
		});

		const unsubs = [
			onStatusChanged((s) => {
				status = s;
			}),
			onOverlayFlash((msg) => {
				clearTimeout(flashTimeout);
				flash = msg;
				flashTimeout = setTimeout(() => (flash = ''), 1000);
			}),
			onSettingsChanged(() => {
				getSettings().then((s) => {
					position = s.overlay_position;
					size = s.overlay_size;
				});
			})
		];
		return () => {
			unsubs.forEach((p) => p.then((fn) => fn()));
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
			cursor: none;
			user-select: none;
		}
	</style>
</svelte:head>

{#if flash}
	<div class="flex {align} {justify} p-3" style="width: 100vw; height: 100vh;">
		<div
			class="flex items-center gap-2 rounded-full border border-white/10 bg-black/60 px-3 py-1.5 shadow-lg backdrop-blur-sm {scale}"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="12"
				height="12"
				viewBox="0 0 24 24"
				fill="none"
				stroke="rgb(74 222 128)"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"><polyline points="20 6 9 17 4 12" /></svg
			>
			<span class="text-xs font-medium text-white">{flash}</span>
		</div>
	</div>
{:else}
	<div class="flex {align} {justify} p-3" style="width: 100vw; height: 100vh;">
		<div
			class="flex items-center gap-2 rounded-full border border-white/10 bg-black/60 px-3 py-1.5 shadow-lg backdrop-blur-sm {scale}"
		>
			{#if status === 'recording'}
				<span class="relative flex h-2.5 w-2.5">
					<span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-red-400/75"
					></span>
					<span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-red-500"></span>
				</span>
				<span class="text-xs font-medium text-white">Recording</span>
			{:else if status === 'processing'}
				<svg
					class="h-3.5 w-3.5 animate-spin text-amber-400"
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
				<svg
					class="h-3.5 w-3.5 animate-spin text-blue-400"
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
			{:else if status === 'idle'}
				<span class="relative flex h-2.5 w-2.5">
					<span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-neutral-400"></span>
				</span>
				<span class="text-xs font-medium text-white/60">Idle</span>
			{/if}
		</div>
	</div>
{/if}
