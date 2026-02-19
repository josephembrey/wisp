<script lang="ts">
	import type { Status } from '$lib/tauri';

	let {
		status,
		flash = '',
		visible = true,
		scale = ''
	}: {
		status: Status;
		flash?: string;
		visible?: boolean;
		scale?: string;
	} = $props();

	type View = Status | 'cancelled' | 'success';
	const view: View = $derived(
		flash === 'Cancelled' ? 'cancelled' : flash ? 'success' : status
	);

	const label = $derived(
		(
			{
				idle: 'Idle',
				recording: 'Recording',
				processing: 'Processing',
				loading: 'Loading',
				cancelled: 'Cancelled',
				success: flash
			} as Record<View, string>
		)[view]
	);

	const textColor = $derived(
		(
			{
				idle: 'rgba(255,255,255,0.6)',
				recording: '#fff',
				processing: '#fff',
				loading: '#fff',
				cancelled: '#fbbf24',
				success: '#4ade80'
			} as Record<View, string>
		)[view]
	);

	const iconKind = $derived<'dot' | 'pulse' | 'spinner' | 'x' | 'check'>(
		(
			{
				idle: 'dot',
				recording: 'pulse',
				processing: 'spinner',
				loading: 'spinner',
				cancelled: 'x',
				success: 'check'
			} as Record<View, 'dot' | 'pulse' | 'spinner' | 'x' | 'check'>
		)[view]
	);

	const dotColor = $derived(view === 'recording' ? '#ef4444' : '#a3a3a3');
	const spinnerColor = $derived(view === 'loading' ? '#60a5fa' : '#fbbf24');

	let innerEl = $state<HTMLDivElement>();
	let pillWidth = $state<number | null>(null);

	$effect(() => {
		label;
		iconKind;
		if (!innerEl) return;
		requestAnimationFrame(() => {
			if (innerEl) pillWidth = innerEl.offsetWidth;
		});
	});
</script>

<div
	class="overflow-hidden rounded-full border border-white/10 bg-black/60 shadow-lg backdrop-blur-sm {scale} {visible ? 'opacity-100' : 'opacity-0'}"
	style="width: {pillWidth !== null ? pillWidth + 'px' : 'auto'}; transition: width 150ms ease-out, opacity 150ms ease-out;"
>
	<div bind:this={innerEl} class="flex w-max items-center gap-2 px-3 py-1.5">
		<div class="flex h-3.5 w-3.5 shrink-0 items-center justify-center">
			{#if iconKind === 'dot' || iconKind === 'pulse'}
				<span class="relative flex h-2.5 w-2.5">
					{#if iconKind === 'pulse'}
						<span
							class="absolute inline-flex h-full w-full animate-ping rounded-full"
							style="background: {dotColor}; opacity: 0.5;"
						></span>
					{/if}
					<span
						class="relative inline-flex h-2.5 w-2.5 rounded-full transition-colors duration-300"
						style="background: {dotColor};"
					></span>
				</span>
			{:else if iconKind === 'spinner'}
				<svg
					class="h-3.5 w-3.5 animate-spin transition-colors duration-300"
					style="color: {spinnerColor};"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle
						class="opacity-25"
						cx="12"
						cy="12"
						r="10"
						stroke="currentColor"
						stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
					></path>
				</svg>
			{:else if iconKind === 'x'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="12"
					height="12"
					viewBox="0 0 24 24"
					fill="none"
					stroke="#fbbf24"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"
					><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
				>
			{:else if iconKind === 'check'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="12"
					height="12"
					viewBox="0 0 24 24"
					fill="none"
					stroke="#4ade80"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"><polyline points="20 6 9 17 4 12" /></svg
				>
			{/if}
		</div>

		<span
			class="text-xs font-medium whitespace-nowrap transition-colors duration-300"
			style="color: {textColor};"
		>
			{label}
		</span>
	</div>
</div>
