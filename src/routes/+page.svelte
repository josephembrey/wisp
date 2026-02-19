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
		onDownloadProgress,
		getGpuBackend,
		getMonitors,
		getInputDevices,
		getModels,
		downloadModel,
		deleteModel,
		isFirstRun,
		resizeWindow as resizeWindowCmd,
		type Settings,
		type Status,
		type MonitorInfo,
		type InputDeviceInfo,
		type ModelInfo,
		type DownloadProgress
	} from '$lib/tauri';
	import { onMount, tick } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { log } from '$lib/log';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import Titlebar from '$lib/components/settings/titlebar.svelte';
	import SettingsGeneral from '$lib/components/settings/general.svelte';
	import SettingsModel from '$lib/components/settings/model.svelte';
	import SettingsOverlay from '$lib/components/settings/overlay.svelte';
	import SettingsAbout from '$lib/components/settings/about.svelte';
	import SettingsTranscribe from '$lib/components/settings/transcribe.svelte';
	import SettingsHistory from '$lib/components/settings/history.svelte';

	let settings: Settings | null = $state(null);
	let status: Status = $state('idle');
	let lastTranscription: string = $state('');
	let contentEl: HTMLDivElement | undefined = $state();
	let tabInnerEl: HTMLDivElement | undefined = $state();
	let gpuBackend: string = $state('');
	let monitors: MonitorInfo[] = $state([]);
	let inputDevices: InputDeviceInfo[] = $state([]);
	let models: ModelInfo[] = $state([]);
	let downloading: string | null = $state(null);
	let downloadProgress: DownloadProgress | null = $state(null);
	let activeTab: string = $state('general');
	let showSaved: boolean = $state(false);
	let flashMessage: string = $state('');
	let savedTimeout: ReturnType<typeof setTimeout> | undefined;
	let flashTimeout: ReturnType<typeof setTimeout> | undefined;

	let lastHeight = 0;
	let tabHeight: number = $state(0);
	let tabAnimated: boolean = $state(false);

	const OUTER_PAD = 16;

	async function resizeWindow() {
		await tick();
		if (!contentEl) return;
		const h = Math.ceil(contentEl.getBoundingClientRect().height) + OUTER_PAD;
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

	$effect(() => {
		if (!tabInnerEl) return;

		const measure = () => {
			const h = tabInnerEl!.scrollHeight;
			if (h > 0) tabHeight = h;
		};

		const ro = new ResizeObserver(measure);
		ro.observe(tabInnerEl);
		measure();
		requestAnimationFrame(() => {
			tabAnimated = true;
		});

		return () => ro.disconnect();
	});

	async function save(updates: Partial<Settings>) {
		if (!settings) return;
		const keys = Object.keys(updates);
		log.info(`[page] save: ${keys.join(', ')}`);
		settings = { ...settings, ...updates };
		try {
			await updateSettings(settings);
			showSavedFlag(true, 750);
		} catch (e) {
			log.error(`[page] save failed: ${e}`);
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

	async function handleDownload(name: string) {
		log.info(`[page] download: ${name}`);
		downloading = name;
		downloadProgress = null;
		try {
			await downloadModel(name);
			log.info(`[page] download complete: ${name}`);
			models = await getModels();
		} catch (e) {
			log.error(`[page] download failed: ${name} ${e}`);
			toast.error(`Failed to download model: ${e}`);
		} finally {
			downloading = null;
			downloadProgress = null;
		}
	}

	async function handleDeleteModel(name: string) {
		log.info(`[page] delete: ${name}`);
		try {
			await deleteModel(name);
			models = await getModels();
		} catch (e) {
			log.error(`[page] delete failed: ${name} ${e}`);
			toast.error(`Failed to delete model: ${e}`);
		}
	}

	onMount(() => {
		log.info('[page] mounted');

		Promise.all([
			getSettings().then((s) => (settings = s)),
			getStatus().then((s) => (status = s)),
			getGpuBackend().then((b) => (gpuBackend = b)),
			getMonitors().then((m) => (monitors = m)),
			getInputDevices().then((d) => (inputDevices = d)),
			getModels().then((m) => (models = m))
		])
			.then(() => log.info('[page] initial data loaded'))
			.catch((e) => log.error(`[page] initial data load error: ${e}`));

		isFirstRun()
			.then((first) => {
				log.info(`[page] isFirstRun: ${first}`);
				if (first) {
					activeTab = 'about';
					handleDownload('base');
				}
			})
			.catch((e) => log.error(`[page] isFirstRun failed: ${e}`));

		const unsubs = [
			onStatusChanged((s) => {
				log.info(`[page] event: status-changed -> ${s}`);
				status = s;
			}),
			onTranscription((t) => {
				log.info(`[page] event: transcription ${t.length} chars`);
				lastTranscription = t;
			}),
			onError((msg) => {
				log.error(`[page] event: backend-error: ${msg}`);
				toast.error(msg);
			}),
			onSettingsChanged((s) => {
				log.info('[page] event: settings-changed');
				settings = s;
			}),
			onOverlayFlash((msg) => {
				log.info(`[page] event: overlay-flash: ${msg}`);
				clearTimeout(flashTimeout);
				flashMessage = msg;
				flashTimeout = setTimeout(() => (flashMessage = ''), 1000);
			}),
			onDownloadProgress((p) => (downloadProgress = p))
		];

		return () => {
			unsubs.forEach((p) => p.then((fn) => fn()));
		};
	});

</script>

<div class="p-2">
<div bind:this={contentEl} class="rounded-xl border border-border bg-card shadow-md overflow-hidden">
	<Titlebar {status} {showSaved} downloading={downloading !== null} {flashMessage} autostart={settings?.autostart ?? false} onautostart={(v) => save({ autostart: v })} />

	{#if settings}
		<Tabs.Root bind:value={activeTab}>
			<div class="px-3 pb-2">
				<Tabs.List class="w-full">
					<Tabs.Trigger value="general">General</Tabs.Trigger>
					<Tabs.Trigger value="model">Model</Tabs.Trigger>
					<Tabs.Trigger value="overlay">Overlay</Tabs.Trigger>
					<Tabs.Trigger value="transcribe">Scribe</Tabs.Trigger>
					<Tabs.Trigger value="history">History</Tabs.Trigger>
					<Tabs.Trigger value="about">About</Tabs.Trigger>
				</Tabs.List>
			</div>

			<div
				class="overflow-hidden"
				class:transition-[height]={tabAnimated}
				class:duration-200={tabAnimated}
				class:ease-out={tabAnimated}
				style:height={tabHeight ? `${tabHeight}px` : 'auto'}
			>
				<div bind:this={tabInnerEl} class="px-3 pb-3">
					<Tabs.Content value="general">
						<SettingsGeneral
							{settings}
							{inputDevices}
							{lastTranscription}
							{showSaved}
							onsave={save}
							onsavedflag={showSavedFlag}
						/>
					</Tabs.Content>

					<Tabs.Content value="model">
						<SettingsModel
							{settings}
							{gpuBackend}
							{models}
							{downloading}
							progress={downloadProgress}
							onsave={save}
							ondownload={handleDownload}
							ondelete={handleDeleteModel}
						/>
					</Tabs.Content>

					<Tabs.Content value="overlay">
						<SettingsOverlay {settings} {monitors} onsave={save} />
					</Tabs.Content>

					<Tabs.Content value="transcribe">
						<SettingsTranscribe {settings} />
					</Tabs.Content>

					<Tabs.Content value="history">
						<SettingsHistory />
					</Tabs.Content>

					<Tabs.Content value="about">
						<SettingsAbout hotkey={settings.hotkey} {settings} onsave={save} />
					</Tabs.Content>
				</div>
			</div>
		</Tabs.Root>
	{/if}
</div>
</div>
