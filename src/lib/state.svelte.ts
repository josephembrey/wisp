import { toast } from 'svelte-sonner';
import { error as logError } from '@tauri-apps/plugin-log';
import {
	getSettings,
	updateSettings,
	getGpuBackend,
	getMonitors,
	getInputDevices,
	getModels,
	downloadModel as downloadModelCmd,
	deleteModel as deleteModelCmd,
	isFirstRun,
	onOverlayState,
	onTranscription,
	onError,
	onSettingsChanged,
	onDownloadProgress,
	type Settings,
	type MonitorInfo,
	type InputDeviceInfo,
	type ModelInfo,
	type DownloadProgress
} from '$lib/tauri';
import { overlay } from '$lib/overlay.svelte';

// App state (reactive via Svelte 5 runes)
let settings: Settings | null = $state(null);
let models: ModelInfo[] = $state([]);
let gpuBackend: string = $state('');
let monitors: MonitorInfo[] = $state([]);
let inputDevices: InputDeviceInfo[] = $state([]);
let downloadProgress: DownloadProgress | null = $state(null);
let lastTranscription: string = $state('');
let activeTab: string = $state('general');

// Actions
async function save(updates: Partial<Settings>) {
	if (!settings) return;
	const dominated = Object.entries(updates).every(
		([k, v]) => (settings as Record<string, unknown>)[k] === v
	);
	if (dominated) return;
	settings = { ...settings, ...updates };
	try {
		await updateSettings(settings);
	} catch (e) {
		logError(`[settings] save failed: ${e}`);
		toast.error(`Failed to save settings: ${e}`);
	}
}

async function downloadModel(name: string) {
	downloadProgress = { model: name, downloaded: 0, total: 0 };
	try {
		await downloadModelCmd(name);
		models = await getModels();
	} catch (e) {
		logError(`[model] download failed: ${e}`);
		toast.error(`Failed to download model: ${e}`);
	} finally {
		downloadProgress = null;
	}
}

async function deleteModel(name: string) {
	try {
		await deleteModelCmd(name);
		models = await getModels();
	} catch (e) {
		logError(`[model] delete failed: ${e}`);
		toast.error(`Failed to delete model: ${e}`);
	}
}

// Init: fetch all state + subscribe to events. Returns cleanup function.
function init(): () => void {
	Promise.all([
		getSettings().then((s) => (settings = s)),
		getGpuBackend().then((b) => (gpuBackend = b)),
		getMonitors().then((m) => (monitors = m)),
		getInputDevices().then((d) => (inputDevices = d)),
		getModels().then((m) => (models = m))
	]).catch((e) => logError(`[init] failed to load state: ${e}`));

	isFirstRun()
		.then((first) => {
			if (first) {
				activeTab = 'about';
				downloadModel('base');
			}
		})
		.catch((e) => logError(`[init] first-run check failed: ${e}`));

	const unsubs = [
		onOverlayState((s) => overlay.push(s)),
		onTranscription((t) => (lastTranscription = t)),
		onError((msg) => {
			logError(`[backend] ${msg}`);
			toast.error(msg);
		}),
		onSettingsChanged((s) => (settings = s)),
		onDownloadProgress((p) => (downloadProgress = p))
	];

	return () => {
		unsubs.forEach((p) => p.then((fn) => fn()));
	};
}

// Export reactive getters + actions
export const app = {
	get settings() {
		return settings;
	},
	get models() {
		return models;
	},
	get gpuBackend() {
		return gpuBackend;
	},
	get monitors() {
		return monitors;
	},
	get inputDevices() {
		return inputDevices;
	},
	get downloadProgress() {
		return downloadProgress;
	},
	get lastTranscription() {
		return lastTranscription;
	},
	get activeTab() {
		return activeTab;
	},
	set activeTab(v: string) {
		activeTab = v;
	},
	save,
	downloadModel,
	deleteModel,
	init
};
