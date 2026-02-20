<script lang="ts">
	import { onMount } from 'svelte';
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import {
		getSettings,
		getStatus,
		onStatusChanged,
		onOverlayFlash,
		onSettingsChanged,
		type Status
	} from '$lib/tauri';
	import { log } from '$lib/log';
	import OverlayPill from '$lib/components/overlay/pill.svelte';

	let status = $state<Status>('idle');
	let flash = $state('');
	let position = $state('top-center');
	let size = $state('medium');
	let alwaysShow = $state(false);
	let ready = $state(false);
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

	const win = getCurrentWebviewWindow();

	$effect(() => {
		if (!ready) return;
		if (visible) {
			win.show();
		} else {
			win.hide();
		}
	});

	onMount(() => {
		log.info('[overlay] mounted');

		Promise.all([
			getSettings().then((s) => {
				log.info(`[overlay] settings loaded, position=${s.overlay_position}`);
				position = s.overlay_position;
				size = s.overlay_size;
				alwaysShow = s.overlay_always_show;
			}),
			getStatus().then((s) => {
				log.info(`[overlay] status: ${s}`);
				status = s;
			})
		])
			.then(() => { ready = true; })
			.catch((e) => log.error(`[overlay] init failed: ${e}`));

		const unsubs = [
			onStatusChanged((s) => {
				log.info(`[overlay] event: status-changed -> ${s}`);
				status = s;
			}),
			onOverlayFlash((msg) => {
				log.info(`[overlay] event: overlay-flash: ${msg}`);
				clearTimeout(flashTimeout);
				flash = msg;
				flashTimeout = setTimeout(() => (flash = ''), 1000);
			}),
			onSettingsChanged((s) => {
				log.info('[overlay] event: settings-changed');
				position = s.overlay_position;
				size = s.overlay_size;
				alwaysShow = s.overlay_always_show;
			})
		];
		return () => {
			log.info('[overlay] unmounting');
			unsubs.forEach((p) => p.then((fn) => fn()));
		};
	});
</script>

<div class="flex {align} {justify} p-3" style="width: 100vw; height: 100vh;">
	<OverlayPill {status} {flash} {visible} {scale} />
</div>
