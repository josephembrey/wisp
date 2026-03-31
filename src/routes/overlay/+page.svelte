<script lang="ts">
	import { onMount } from 'svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import ClipboardIcon from '@lucide/svelte/icons/clipboard';
	import TextCursorIcon from '@lucide/svelte/icons/text-cursor';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import XIcon from '@lucide/svelte/icons/x';
	import { error as logError } from '@tauri-apps/plugin-log';
	import { getSettings, onSettingsChanged, type Settings } from '$lib/tauri';
	import { overlay } from '$lib/overlay.svelte';

	// Overlay state
	let current = $derived(overlay.current);

	// Settings-driven state
	let alwaysShow = $state(false);
	let position = $state('top-right');
	let size = $state('md');
	let outputMode = $state('paste');

	const align = $derived(position.startsWith('bottom') ? 'items-end' : 'items-start');
	const justify = $derived(
		{ left: 'justify-start', right: 'justify-end' }[position.split('-')[1]] ?? 'justify-center'
	);
	const sizeClasses = $derived(
		{
			small: 'gap-1.5 px-2 py-1 text-[0.625rem]',
			large: 'gap-3 px-4 py-2.5 text-base'
		}[size] ?? 'gap-2 px-3 py-1.5 text-xs'
	);
	const iconSize = $derived({ small: 10, large: 18 }[size] ?? 14);
	const dotSize = $derived({ small: 'h-2 w-2', large: 'h-3.5 w-3.5' }[size] ?? 'h-2.5 w-2.5');
	const visible = $derived(current.status !== 'idle' || alwaysShow);

	// Settings sync
	function applySettings(s: Settings) {
		alwaysShow = s.overlay_always_show ?? false;
		position = s.overlay_position ?? 'top-right';
		size = s.overlay_size ?? 'md';
		outputMode = s.output_mode ?? 'paste';
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
		class="flex items-center rounded-full border border-white/10 shadow-lg backdrop-blur-sm transition-opacity duration-150 {sizeClasses}"
		style:background="var(--overlay-bg)"
		class:opacity-0={!visible}
	>
		<!-- Status icon + label per state -->
		<!-- idle: muted gray dot -->
		{#if current.status === 'idle'}
			<span class="relative flex {dotSize}">
				<span
					class="relative inline-flex {dotSize} rounded-full"
					style:background="var(--overlay-idle)"
				></span>
			</span>
			<span class="overlay-label" style:color="var(--overlay-text-muted)">Idle</span>

			<!-- recording: pulsing red dot -->
		{:else if current.status === 'recording'}
			<span class="relative flex {dotSize}">
				<span
					class="absolute inline-flex h-full w-full animate-ping rounded-full"
					style:background="var(--overlay-recording)"
				></span>
				<span
					class="relative inline-flex {dotSize} rounded-full"
					style:background="var(--overlay-recording)"
				></span>
			</span>
			<span class="overlay-label" style:color="var(--overlay-text)">Recording</span>

			<!-- processing/loading: amber spinner -->
		{:else if current.status === 'processing' || current.status === 'loading'}
			<Loader2Icon size={iconSize} class="animate-spin" color="var(--overlay-processing)" />
			<span class="overlay-label" style:color="var(--overlay-text)">
				{current.status === 'loading' ? 'Loading' : 'Processing'}
			</span>

			<!-- cancelled: amber x -->
		{:else if current.status === 'cancelled'}
			<XIcon size={iconSize} color="var(--overlay-processing)" strokeWidth={2.5} />
			<span class="overlay-label" style:color="var(--overlay-processing)">Cancelled</span>

			<!-- mode change: show new output mode -->
		{:else if current.status === 'output_mode'}
			{#if outputMode === 'clipboard'}
				<ClipboardIcon size={iconSize} color="var(--overlay-text)" strokeWidth={2.5} />
				<span class="overlay-label" style:color="var(--overlay-text)">Clipboard</span>
			{:else}
				<TextCursorIcon size={iconSize} color="var(--overlay-text)" strokeWidth={2.5} />
				<span class="overlay-label" style:color="var(--overlay-text)">Cursor</span>
			{/if}

			<!-- saved/copied/typed/deleted: green check -->
		{:else}
			<CheckIcon size={iconSize} color="var(--overlay-success)" strokeWidth={2.5} />
			<span class="overlay-label" style:color="var(--overlay-success)">
				{current.status === 'copied'
					? 'Copied'
					: current.status === 'typed'
						? 'Typed'
						: current.status === 'deleted'
							? 'Deleted'
							: 'Saved'}
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
		font-size: inherit;
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
