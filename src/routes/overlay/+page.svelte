<script lang="ts">
	import { onMount } from 'svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import XIcon from '@lucide/svelte/icons/x';
	import { error as logError } from '@tauri-apps/plugin-log';
	import { getSettings, onSettingsChanged, type Settings, type OverlayStatus } from '$lib/tauri';
	import { overlay } from '$lib/overlay.svelte';

	const colors: Record<OverlayStatus, { textColor: string; iconColor: string }> = {
		idle: { textColor: 'var(--overlay-text-muted)', iconColor: 'var(--overlay-idle)' },
		recording: { textColor: 'var(--overlay-text)', iconColor: 'var(--overlay-recording)' },
		processing: { textColor: 'var(--overlay-text)', iconColor: 'var(--overlay-processing)' },
		loading: { textColor: 'var(--overlay-text)', iconColor: 'var(--overlay-processing)' },
		success: { textColor: 'var(--overlay-success)', iconColor: 'var(--overlay-success)' },
		cancelled: { textColor: 'var(--overlay-processing)', iconColor: 'var(--overlay-processing)' }
	};

	let alwaysShow = $state(false);
	let position = $state('top-right');
	let size = $state('md');
	let current = $derived(overlay.current);
	let color = $derived(colors[current.status]);

	const align = $derived(position.startsWith('bottom') ? 'items-end' : 'items-start');
	const justify = $derived(
		{ left: 'justify-start', right: 'justify-end' }[position.split('-')[1]] ?? 'justify-center'
	);
	const scale = $derived({ small: 'scale-75', large: 'scale-150' }[size] ?? '');
	const visible = $derived(current.status !== 'idle' || alwaysShow);

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

<div class="flex {align} {justify} h-screen w-screen p-3">
	<div
		class="flex items-center gap-2 rounded-full border border-white/10 px-3 py-1.5 shadow-lg backdrop-blur-sm transition-opacity duration-150 {scale}"
		style:background="var(--overlay-bg)"
		class:opacity-0={!visible}
	>
		{#if current.status === 'idle' || current.status === 'recording'}
			<span class="relative flex h-2.5 w-2.5">
				{#if current.status === 'recording'}
					<span
						class="absolute inline-flex h-full w-full animate-ping rounded-full"
						style:background={color.iconColor}
					></span>
				{/if}
				<span
					class="relative inline-flex h-2.5 w-2.5 rounded-full"
					style:background={color.iconColor}
				></span>
			</span>
		{:else if current.status === 'processing' || current.status === 'loading'}
			<Loader2Icon size={14} class="animate-spin" color={color.iconColor} />
		{:else if current.status === 'cancelled'}
			<XIcon size={12} color={color.iconColor} strokeWidth={2.5} />
		{:else}
			<CheckIcon size={12} color={color.iconColor} strokeWidth={2.5} />
		{/if}

		<span
			class="text-xs font-medium whitespace-nowrap transition-colors duration-300"
			style:color={color.textColor}
		>
			{current.label}
		</span>
	</div>
</div>

<style>
	:root {
		--overlay-bg: rgba(0, 0, 0, 0.6);
		--overlay-text: #fff;
		--overlay-text-muted: rgba(255, 255, 255, 0.6);
		--overlay-idle: #a3a3a3;
		--overlay-recording: #ef4444;
		--overlay-processing: #fbbf24;
		--overlay-success: #4ade80;
	}

	:global(html),
	:global(body) {
		background: transparent !important;
		overflow: hidden;
		pointer-events: none;
		cursor: none;
		user-select: none;
	}
</style>
