<script lang="ts">
	import { onMount } from 'svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import XIcon from '@lucide/svelte/icons/x';
	import {
		getSettings,
		getOverlayState,
		onOverlayState,
		onSettingsChanged,
		type Settings,
		type OverlayState,
		type OverlayIcon
	} from '$lib/tauri';

	const styles: Record<OverlayIcon, { color: string; iconColor: string }> = {
		dot: { color: 'var(--overlay-text-muted)', iconColor: 'var(--overlay-idle)' },
		pulse: { color: 'var(--overlay-text)', iconColor: 'var(--overlay-recording)' },
		spinner: { color: 'var(--overlay-text)', iconColor: 'var(--overlay-processing)' },
		check: { color: 'var(--overlay-success)', iconColor: 'var(--overlay-success)' },
		x: { color: 'var(--overlay-processing)', iconColor: 'var(--overlay-processing)' }
	};

	let alwaysShow = $state(false);
	let position = $state('top-right');
	let size = $state('md');
	let overlay = $state<OverlayState>({ icon: 'dot', label: 'Idle', ttl_ms: null });
	let ttlTimeout: ReturnType<typeof setTimeout> | undefined;

	const style = $derived(styles[overlay.icon]);
	const align = $derived(position.startsWith('bottom') ? 'items-end' : 'items-start');
	const justify = $derived(
		{ left: 'justify-start', right: 'justify-end' }[position.split('-')[1]] ?? 'justify-center'
	);
	const scale = $derived({ small: 'scale-75', large: 'scale-150' }[size] ?? '');
	const visible = $derived(overlay.icon !== 'dot' || alwaysShow);

	function applySettings(s: Settings) {
		alwaysShow = s.overlay_always_show ?? false;
		position = s.overlay_position ?? 'top-right';
		size = s.overlay_size ?? 'md';
	}

	onMount(() => {
		getSettings().then(applySettings);
		getOverlayState().then((s) => (overlay = s));

		const unsubs = [
			onOverlayState((s) => {
				clearTimeout(ttlTimeout);
				overlay = s;
				if (s.ttl_ms != null) {
					ttlTimeout = setTimeout(() => {
						getOverlayState().then((real) => (overlay = real));
					}, s.ttl_ms);
				}
			}),
			onSettingsChanged(applySettings)
		];
		return () => unsubs.forEach((p) => p.then((fn) => fn()));
	});
</script>

<div class="flex {align} {justify} h-screen w-screen p-3">
	<div
		class="flex items-center gap-2 rounded-full border border-white/10 px-3 py-1.5 shadow-lg backdrop-blur-sm transition-opacity duration-150 {scale}"
		style:background="var(--overlay-bg)"
		class:opacity-0={!visible}
	>
		{#if overlay.icon === 'dot' || overlay.icon === 'pulse'}
			<span class="relative flex h-2.5 w-2.5">
				{#if overlay.icon === 'pulse'}
					<span
						class="absolute inline-flex h-full w-full animate-ping rounded-full"
						style:background={style.iconColor}
					></span>
				{/if}
				<span
					class="relative inline-flex h-2.5 w-2.5 rounded-full"
					style:background={style.iconColor}
				></span>
			</span>
		{:else if overlay.icon === 'spinner'}
			<Loader2Icon size={14} class="animate-spin" color={style.iconColor} />
		{:else if overlay.icon === 'x'}
			<XIcon size={12} color={style.iconColor} strokeWidth={2.5} />
		{:else}
			<CheckIcon size={12} color={style.iconColor} strokeWidth={2.5} />
		{/if}

		<span
			class="text-xs font-medium whitespace-nowrap transition-colors duration-300"
			style:color={style.color}
		>
			{overlay.label}
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
