import { toast } from 'svelte-sonner';
import { error as logError } from '@tauri-apps/plugin-log';
import {
	getSettings,
	updateSettings,
	getGpuBackend,
	getMonitors,
	getInputDevices,
	getModels,
	getMemoryInfo,
	downloadModel as downloadModelCmd,
	deleteModel as deleteModelCmd,
	isFirstRun,
	onTranscription,
	onError,
	onSettingsChanged,
	onDownloadProgress,
	type Settings,
	type MonitorInfo,
	type InputDeviceInfo,
	type ModelInfo,
	type MemoryInfo,
	type DownloadProgress
} from '$lib/tauri';

// Reactive app state — consumed via the `app` export below
let settings: Settings | null = $state(null);
let models: ModelInfo[] = $state([]);
let gpuBackend: string = $state('');
let monitors: MonitorInfo[] = $state([]);
let inputDevices: InputDeviceInfo[] = $state([]);
let memoryInfo: MemoryInfo = $state({ total_mb: 0, available_mb: 0 });
let downloadProgress: DownloadProgress | null = $state(null);
let lastTranscription: string = $state('');
let activeTab: string = $state('general');

// Actions — optimistic updates with rollback on failure
async function save(updates: Partial<Settings>) {
	if (!settings) return;
	const unchanged = Object.entries(updates).every(
		([k, v]) => (settings as Record<string, unknown>)[k] === v
	);
	if (unchanged) return;
	const prev = settings;
	settings = { ...settings, ...updates };
	try {
		await updateSettings(settings);
	} catch (e) {
		settings = prev;
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

// Fetch all state + subscribe to backend events. Returns cleanup function.
function init(): () => void {
	Promise.all([
		getSettings().then((s) => (settings = s)),
		getGpuBackend().then((b) => (gpuBackend = b)),
		getMonitors().then((m) => (monitors = m)),
		getInputDevices().then((d) => (inputDevices = d)),
		getModels().then((m) => (models = m)),
		getMemoryInfo(false).then((m) => (memoryInfo = m))
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
		onTranscription((t) => (lastTranscription = t)),
		onError((msg) => {
			logError(`[backend] ${msg}`);
			toast.error(msg);
		}),
		onSettingsChanged((s) => {
			const gpuChanged = s.gpu !== settings?.gpu;
			settings = s;
			if (gpuChanged) getMemoryInfo(s.gpu ?? false).then((m) => (memoryInfo = m));
		}),
		onDownloadProgress((p) => (downloadProgress = p))
	];

	return () => {
		unsubs.forEach((p) => p.then((fn) => fn()));
	};
}

// Public API — reactive getters + actions
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
	get memoryInfo() {
		return memoryInfo;
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
