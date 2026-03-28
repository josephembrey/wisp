<script lang="ts">
	import { onMount } from 'svelte';
	import { log } from '$lib/log';
	import CheckIcon from '@lucide/svelte/icons/check';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import XIcon from '@lucide/svelte/icons/x';
	import {
		getSettings,
		getStatus,
		onStatusChanged,
		onOverlayFlash,
		onSettingsChanged,
		type Settings,
		type Status
	} from '$lib/tauri';

	// View config — the truth table
	type View = Status | 'cancelled' | 'success';
	const views: Record<View, { label: string; color: string; icon: string; iconColor: string }> = {
		idle: { label: 'Idle', color: 'rgba(255,255,255,0.6)', icon: 'dot', iconColor: '#a3a3a3' },
		loading: { label: 'Loading', color: '#fff', icon: 'spinner', iconColor: '#60a5fa' },
		recording: { label: 'Recording', color: '#fff', icon: 'pulse', iconColor: '#ef4444' },
		processing: { label: 'Processing', color: '#fff', icon: 'spinner', iconColor: '#fbbf24' },
		cancelled: { label: 'Cancelled', color: '#fbbf24', icon: 'x', iconColor: '#fbbf24' },
		success: { label: '', color: '#4ade80', icon: 'check', iconColor: '#4ade80' }
	};

	// Settings (overwritten on load)
	let alwaysShow = $state(false);
	let position = $state('top-right');
	let size = $state('md');

	// Runtime
	let flash = $state('');
	let flashTimeout: ReturnType<typeof setTimeout> | undefined;
	let status = $state<Status>('idle');

	// Derived
	const view: View = $derived(flash === 'Cancelled' ? 'cancelled' : flash ? 'success' : status);
	const cfg = $derived(views[view]);
	const label = $derived(cfg.label || flash);
	const align = $derived(position.startsWith('bottom') ? 'items-end' : 'items-start');
	const justify = $derived(
		{ left: 'justify-start', right: 'justify-end' }[position.split('-')[1]] ?? 'justify-center'
	);
	const scale = $derived({ small: 'scale-75', large: 'scale-150' }[size] ?? '');
	const visible = $derived(flash !== '' || status !== 'idle' || alwaysShow);

	function applySettings(s: Settings) {
		alwaysShow = s.overlay_always_show ?? false;
		position = s.overlay_position ?? 'top-right';
		size = s.overlay_size ?? 'md';
	}

	onMount(() => {
		log.info('[overlay] mounted');
		getSettings().then((s) => {
			log.info(`[overlay] loaded, position=${s.overlay_position}`);
			applySettings(s);
		});
		getStatus().then((s) => (status = s));

		const unsubs = [
			onStatusChanged((s) => (status = s)),
			onSettingsChanged(applySettings),
			onOverlayFlash((msg) => {
				clearTimeout(flashTimeout);
				flash = msg;
				flashTimeout = setTimeout(() => (flash = ''), 1000);
			})
		];
		return () => unsubs.forEach((p) => p.then((fn) => fn()));
	});
</script>

<div class="flex {align} {justify} h-screen w-screen p-3">
	<div
		class="flex items-center gap-2 rounded-full border border-white/10 bg-black/60 px-3 py-1.5 shadow-lg backdrop-blur-sm transition-opacity duration-150 {scale}"
		class:opacity-0={!visible}
	>
		{#if cfg.icon === 'dot' || cfg.icon === 'pulse'}
			<span class="relative flex h-2.5 w-2.5">
				{#if cfg.icon === 'pulse'}
					<span
						class="absolute inline-flex h-full w-full animate-ping rounded-full"
						style:background={cfg.iconColor}
					></span>
				{/if}
				<span class="relative inline-flex h-2.5 w-2.5 rounded-full" style:background={cfg.iconColor}
				></span>
			</span>
		{:else if cfg.icon === 'spinner'}
			<Loader2Icon size={14} class="animate-spin" color={cfg.iconColor} />
		{:else if cfg.icon === 'x'}
			<XIcon size={12} color={cfg.iconColor} strokeWidth={2.5} />
		{:else}
			<CheckIcon size={12} color={cfg.iconColor} strokeWidth={2.5} />
		{/if}

		<span
			class="text-xs font-medium whitespace-nowrap transition-colors duration-300"
			style:color={cfg.color}
		>
			{label}
		</span>
	</div>
</div>

<style>
	:global(html),
	:global(body) {
		background: transparent !important;
		overflow: hidden;
		pointer-events: none;
		cursor: none;
		user-select: none;
	}
</style>
