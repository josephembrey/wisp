import { toast } from 'svelte-sonner';
import {
	getSettings,
	updateSettings,
	getOverlayState,
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
	type OverlayState,
	type OverlayIcon,
	type MonitorInfo,
	type InputDeviceInfo,
	type ModelInfo,
	type DownloadProgress
} from '$lib/tauri';

// App state (reactive via Svelte 5 runes)
let settings: Settings | null = $state(null);
let overlay: OverlayState = $state({ icon: 'dot', label: 'Idle', ttl_ms: null });
let models: ModelInfo[] = $state([]);
let gpuBackend: string = $state('');
let monitors: MonitorInfo[] = $state([]);
let inputDevices: InputDeviceInfo[] = $state([]);
let downloading: string | null = $state(null);
let downloadProgress: DownloadProgress | null = $state(null);
let lastTranscription: string = $state('');
let activeTab: string = $state('general');

// Overlay TTL timer
let overlayTtlTimeout: ReturnType<typeof setTimeout> | undefined;

// Frontend notification gateway — same codepath as backend overlay events
function notify(label: string, icon: OverlayIcon, ttl_ms: number) {
	clearTimeout(overlayTtlTimeout);
	overlay = { icon, label, ttl_ms };
	overlayTtlTimeout = setTimeout(() => {
		getOverlayState().then((real) => (overlay = real));
	}, ttl_ms);
}

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
		toast.error(`Failed to save settings: ${e}`);
	}
}

async function downloadModel(name: string) {
	downloading = name;
	downloadProgress = null;
	try {
		await downloadModelCmd(name);
		models = await getModels();
	} catch (e) {
		toast.error(`Failed to download model: ${e}`);
	} finally {
		downloading = null;
		downloadProgress = null;
	}
}

async function deleteModel(name: string) {
	try {
		await deleteModelCmd(name);
		models = await getModels();
	} catch (e) {
		toast.error(`Failed to delete model: ${e}`);
	}
}

// Init: fetch all state + subscribe to events. Returns cleanup function.
function init(): () => void {
	Promise.all([
		getSettings().then((s) => (settings = s)),
		getOverlayState().then((s) => (overlay = s)),
		getGpuBackend().then((b) => (gpuBackend = b)),
		getMonitors().then((m) => (monitors = m)),
		getInputDevices().then((d) => (inputDevices = d)),
		getModels().then((m) => (models = m))
	]);

	isFirstRun()
		.then((first) => {
			if (first) {
				activeTab = 'about';
				downloadModel('base');
			}
		})
		.catch(() => {});

	const unsubs = [
		onOverlayState((s) => {
			clearTimeout(overlayTtlTimeout);
			overlay = s;
			if (s.ttl_ms != null) {
				overlayTtlTimeout = setTimeout(() => {
					getOverlayState().then((real) => (overlay = real));
				}, s.ttl_ms);
			}
		}),
		onTranscription((t) => (lastTranscription = t)),
		onError((msg) => toast.error(msg)),
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
	get overlay() {
		return overlay;
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
	get downloading() {
		return downloading;
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
	notify,
	downloadModel,
	deleteModel,
	init
};
