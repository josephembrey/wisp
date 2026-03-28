<script lang="ts">
	import { onMount } from 'svelte';
	import { log } from '$lib/log';
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
		dot: { color: 'rgba(255,255,255,0.6)', iconColor: '#a3a3a3' },
		pulse: { color: '#fff', iconColor: '#ef4444' },
		spinner: { color: '#fff', iconColor: '#fbbf24' },
		check: { color: '#4ade80', iconColor: '#4ade80' },
		x: { color: '#fbbf24', iconColor: '#fbbf24' }
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
		log.info('[overlay] mounted');
		getSettings().then((s) => {
			log.info(`[overlay] loaded, position=${s.overlay_position}`);
			applySettings(s);
		});
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
		class="flex items-center gap-2 rounded-full border border-white/10 bg-black/60 px-3 py-1.5 shadow-lg backdrop-blur-sm transition-opacity duration-150 {scale}"
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
	:global(html),
	:global(body) {
		background: transparent !important;
		overflow: hidden;
		pointer-events: none;
		cursor: none;
		user-select: none;
	}
</style>
