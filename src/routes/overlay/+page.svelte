<script lang="ts">
	import { onMount } from 'svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import XIcon from '@lucide/svelte/icons/x';
	import { error as logError } from '@tauri-apps/plugin-log';
	import { getSettings, onSettingsChanged, type Settings } from '$lib/tauri';
	import { overlay } from '$lib/overlay.svelte';

	// Overlay state
	let current = $derived(overlay.current);

	// Position & visibility (from settings)
	let alwaysShow = $state(false);
	let position = $state('top-right');
	let size = $state('md');

	const align = $derived(position.startsWith('bottom') ? 'items-end' : 'items-start');
	const justify = $derived(
		{ left: 'justify-start', right: 'justify-end' }[position.split('-')[1]] ?? 'justify-center'
	);
	const overlayScale = $derived({ small: 'scale-75', large: 'scale-150' }[size] ?? '');
	const visible = $derived(current.status !== 'idle' || alwaysShow);

	// Settings sync
	function applySettings(s: Settings) {
		alwaysShow = s.overlay_always_show ?? false;
		position = s.overlay_position ?? 'top-right';
		size = s.overlay_size ?? 'md';
	}

	onMount(() => {
		getSettings()
			.then(applySettings)
			.catch((e) => logError(`[overlay] settings load failed: ${e}`));

		const unsub = onSettingsChanged(applySettings);
		return () => unsub.then((fn) => fn());
	});
</script>

<!-- Overlay pill: positioned fullscreen, fades in/out -->
<div class="flex {align} {justify} h-screen w-screen p-3">
	<div
		class="flex items-center gap-2 rounded-full border border-white/10 px-3 py-1.5 shadow-lg backdrop-blur-sm transition-opacity duration-150 {overlayScale}"
		style:background="var(--overlay-bg)"
		class:opacity-0={!visible}
	>
		<!-- Status icon + label per state -->
		<!-- idle: muted gray dot -->
		{#if current.status === 'idle'}
			<span class="relative flex h-2.5 w-2.5">
				<span
					class="relative inline-flex h-2.5 w-2.5 rounded-full"
					style:background="var(--overlay-idle)"
				></span>
			</span>
			<span class="overlay-label" style:color="var(--overlay-text-muted)">Idle</span>

			<!-- recording: pulsing red dot -->
		{:else if current.status === 'recording'}
			<span class="relative flex h-2.5 w-2.5">
				<span
					class="absolute inline-flex h-full w-full animate-ping rounded-full"
					style:background="var(--overlay-recording)"
				></span>
				<span
					class="relative inline-flex h-2.5 w-2.5 rounded-full"
					style:background="var(--overlay-recording)"
				></span>
			</span>
			<span class="overlay-label" style:color="var(--overlay-text)">Recording</span>

			<!-- processing/loading: amber spinner -->
		{:else if current.status === 'processing' || current.status === 'loading'}
			<Loader2Icon size={14} class="animate-spin" color="var(--overlay-processing)" />
			<span class="overlay-label" style:color="var(--overlay-text)">
				{current.status === 'loading' ? 'Loading' : 'Processing'}
			</span>

			<!-- cancelled: amber x -->
		{:else if current.status === 'cancelled'}
			<XIcon size={12} color="var(--overlay-processing)" strokeWidth={2.5} />
			<span class="overlay-label" style:color="var(--overlay-processing)">Cancelled</span>

			<!-- saved/copied/typed/deleted: green check -->
		{:else}
			<CheckIcon size={12} color="var(--overlay-success)" strokeWidth={2.5} />
			<span class="overlay-label" style:color="var(--overlay-success)">
				{current.status === 'saved'
					? 'Saved'
					: current.status === 'copied'
						? 'Copied'
						: current.status === 'typed'
							? 'Typed'
							: 'Deleted'}
			</span>
		{/if}
	</div>
</div>

<style>
	/* Overlay color tokens (separate window, can't use app theme) */
	:root {
		--overlay-bg: rgba(0, 0, 0, 0.6);
		--overlay-text: #fff;
		--overlay-text-muted: rgba(255, 255, 255, 0.6);
		--overlay-idle: #a3a3a3;
		--overlay-recording: #ef4444;
		--overlay-processing: #fbbf24;
		--overlay-success: #4ade80;
	}

	.overlay-label {
		font-size: 0.75rem;
		font-weight: 500;
		white-space: nowrap;
		transition: color 300ms;
	}

	/* Transparent window setup */
	:global(html),
	:global(body) {
		background: transparent !important;
		overflow: hidden;
		pointer-events: none;
		cursor: none;
		user-select: none;
	}
</style>
