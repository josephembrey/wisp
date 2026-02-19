<script lang="ts">
	import {
		getSettings,
		updateSettings,
		getStatus,
		onStatusChanged,
		onTranscription,
		onError,
		onSettingsChanged,
		onOverlayFlash,
		getGpuBackend,
		getMonitors,
		getInputDevices,
		isFirstRun,
		resizeWindow as resizeWindowCmd,
		hotkeyPress,
		hotkeyRelease,
		type Settings,
		type Status,
		type MonitorInfo,
		type InputDeviceInfo
	} from '$lib/tauri';
	import { tick } from 'svelte';
	import { toast } from 'svelte-sonner';
	import Titlebar from '$lib/components/titlebar.svelte';
	import NavSidebar from '$lib/components/nav-sidebar.svelte';
	import SettingsMain from '$lib/components/settings-main.svelte';
	import SettingsAdvanced from '$lib/components/settings-advanced.svelte';
	import SettingsOverlay from '$lib/components/settings-overlay.svelte';
	import SettingsAbout from '$lib/components/settings-about.svelte';

	let settings: Settings | null = $state(null);
	let status: Status = $state('idle');
	let lastTranscription: string = $state('');
	let contentEl: HTMLDivElement | undefined = $state();
	let gpuBackend: string = $state('');
	let monitors: MonitorInfo[] = $state([]);
	let inputDevices: InputDeviceInfo[] = $state([]);
	let isDownloading: boolean = $state(false);
	let activeSection: string = $state('main');
	let showSaved: boolean = $state(false);
	let flashMessage: string = $state('');
	let savedTimeout: ReturnType<typeof setTimeout> | undefined;
	let flashTimeout: ReturnType<typeof setTimeout> | undefined;

	let lastHeight = 0;

	// Auto-resize window to fit content
	async function resizeWindow() {
		await tick();
		if (!contentEl) return;
		const h = Math.ceil(contentEl.getBoundingClientRect().height);
		if (h > 0 && Math.abs(h - lastHeight) >= 2) {
			lastHeight = h;
			resizeWindowCmd(h);
		}
	}

	$effect(() => {
		if (!contentEl) return;

		const ro = new ResizeObserver(() => resizeWindow());
		ro.observe(contentEl);
		resizeWindow();

		return () => ro.disconnect();
	});

	async function save(updates: Partial<Settings>) {
		if (!settings) return;
		settings = { ...settings, ...updates };
		try {
			await updateSettings(settings);
			showSavedFlag(true, 750);
		} catch (e) {
			toast.error(`Failed to save settings: ${e}`);
		}
	}

	function showSavedFlag(show: boolean, timeout?: number) {
		clearTimeout(savedTimeout);
		showSaved = show;
		if (show && timeout) {
			savedTimeout = setTimeout(() => (showSaved = false), timeout);
		}
	}

	$effect(() => {
		getSettings().then((s) => (settings = s));
		getStatus().then((s) => (status = s));
		getGpuBackend().then((b) => (gpuBackend = b));
		getMonitors().then((m) => (monitors = m));
		getInputDevices().then((d) => (inputDevices = d));
		isFirstRun().then((first) => {
			if (first) activeSection = 'about';
		});

		const unsubs = [
			onStatusChanged((s) => (status = s)),
			onTranscription((t) => (lastTranscription = t)),
			onError((msg) => toast.error(msg)),
			onSettingsChanged(() => {
				getSettings().then((s) => (settings = s));
			}),
			onOverlayFlash((msg) => {
				clearTimeout(flashTimeout);
				flashMessage = msg;
				flashTimeout = setTimeout(() => (flashMessage = ''), 1000);
			})
		];

		return () => {
			unsubs.forEach((p) => p.then((fn) => fn()));
		};
	});

	// JS-side hotkey fallback: rdev doesn't receive key events when WebView2 is focused
	import { mapBrowserKey } from '$lib/keys';

	let pressedKeys = new Set<string>();
	let hotkeyActive = false;

	function handleKeydown(e: KeyboardEvent) {
		const key = mapBrowserKey(e.code);
		const combo = settings?.hotkey?.split('+') || [];
		// Only intercept keys that are part of the hotkey combo
		if (combo.includes(key)) {
			e.preventDefault();
		}
		pressedKeys.add(key);
		if (!hotkeyActive && combo.length > 0 && combo.every((k) => pressedKeys.has(k))) {
			hotkeyActive = true;
			hotkeyPress();
		}
	}

	function handleKeyup(e: KeyboardEvent) {
		const key = mapBrowserKey(e.code);
		if (hotkeyActive && (settings?.hotkey?.split('+') || []).includes(key)) {
			hotkeyActive = false;
			hotkeyRelease();
		}
		pressedKeys.delete(key);
	}
</script>

<svelte:window onkeydown={handleKeydown} onkeyup={handleKeyup} />

<div bind:this={contentEl}>
	<Titlebar {status} {showSaved} downloading={isDownloading} {flashMessage} />

	{#if settings}
		<div class="flex gap-0 p-4">
			<div class="w-28 shrink-0 pr-3">
				<NavSidebar active={activeSection} onnavigate={(s) => (activeSection = s)} />
			</div>
			<div class="min-w-0 flex-1">
				{#if activeSection === 'main'}
					<SettingsMain
						{settings}
						{inputDevices}
						{lastTranscription}
						{showSaved}
						onsave={save}
						onsavedflag={showSavedFlag}
					/>
				{:else if activeSection === 'advanced'}
					<SettingsAdvanced
						{settings}
						{gpuBackend}
						onsave={save}
						ondownloadchange={(v) => (isDownloading = v)}
					/>
				{:else if activeSection === 'overlay'}
					<SettingsOverlay {settings} {monitors} onsave={save} />
				{:else if activeSection === 'about'}
					<SettingsAbout hotkey={settings.hotkey} />
				{/if}
			</div>
		</div>
	{/if}
</div>
