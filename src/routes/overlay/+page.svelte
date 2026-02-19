<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getSettings,
		onStatusChanged,
		onOverlayFlash,
		onSettingsChanged,
		type Status
	} from '$lib/tauri';
	import OverlayPill from '$lib/components/overlay/pill.svelte';

	let status = $state<Status>('idle');
	let flash = $state('');
	let position = $state('top-center');
	let size = $state('medium');
	let alwaysShow = $state(false);
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

	const visible = $derived(
		flash !== '' || status !== 'idle' || alwaysShow
	);

	onMount(() => {
		getSettings().then((s) => {
			position = s.overlay_position;
			size = s.overlay_size;
			alwaysShow = s.overlay_always_show;
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
					alwaysShow = s.overlay_always_show;
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

<div class="flex {align} {justify} p-3" style="width: 100vw; height: 100vh;">
	<OverlayPill {status} {flash} {visible} {scale} />
</div>
