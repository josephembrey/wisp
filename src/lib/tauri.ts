import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import {
	commands,
	type Result,
	type Settings,
	type OverlayState,
	type DownloadProgress
} from './bindings';

export type {
	Settings,
	OverlayState,
	OverlayStatus,
	OverlayPosition,
	OverlaySize,
	ModelInfo,
	InputDeviceInfo,
	MonitorInfo,
	MemoryInfo,
	DownloadProgress,
	OutputMode,
	ModelLoading,
	HistoryEntry
} from './bindings';

// Unwrap specta Result<T, string> — throws on error so callers get plain T
function unwrap<T>(result: Result<T, string>): T {
	if (result.status === 'error') throw new Error(result.error);
	return result.data;
}

// Commands — pass-through for infallible, unwrap for fallible
export const isFirstRun = commands.isFirstRun;
export const getSettings = commands.getSettings;
export const getModels = commands.getModels;
export const getGpuBackend = commands.getGpuBackend;
export const getMonitors = commands.getMonitors;
export const getMemoryInfo = commands.getMemoryInfo;
export const getInputDevices = commands.getInputDevices;
export const resizeWindow = commands.resizeWindow;
export const quit = commands.quit;

export const updateSettings = async (settings: Settings) =>
	unwrap(await commands.updateSettings(settings));
export const downloadModel = async (name: string) => unwrap(await commands.downloadModel(name));
export const deleteModel = async (name: string) => unwrap(await commands.deleteModel(name));
export const resetApp = async () => unwrap(await commands.resetApp());
export const transcribeFile = async (path: string) => unwrap(await commands.transcribeFile(path));
export const getHistory = commands.getHistory;
export const clearHistory = commands.clearHistory;
export const deleteHistoryEntry = commands.deleteHistoryEntry;
export const showLogDir = async () => unwrap(await commands.showLogDir());

// Window helpers
export const hideWindow = () => getCurrentWindow().hide();
export const minimizeWindow = () => getCurrentWindow().minimize();

// Event listeners — each returns Promise<UnlistenFn> for cleanup
export const onOverlayState = (cb: (state: OverlayState) => void): Promise<UnlistenFn> =>
	listen<OverlayState>('overlay-state', (e) => cb(e.payload));

export const onDownloadProgress = (cb: (progress: DownloadProgress) => void): Promise<UnlistenFn> =>
	listen<DownloadProgress>('download-progress', (e) => cb(e.payload));

export const onTranscription = (cb: (text: string) => void): Promise<UnlistenFn> =>
	listen<string>('transcription', (e) => cb(e.payload));

export const onError = (cb: (message: string) => void): Promise<UnlistenFn> =>
	listen<string>('backend-error', (e) => cb(e.payload));

export const onSettingsChanged = (cb: (settings: Settings) => void): Promise<UnlistenFn> =>
	listen<Settings>('settings-changed', (e) => cb(e.payload));

export const onTranscribeFileProgress = (cb: (status: string) => void): Promise<UnlistenFn> =>
	listen<string>('transcribe-file-progress', (e) => cb(e.payload));

export const onHistoryChanged = (cb: () => void): Promise<UnlistenFn> =>
	listen('history-changed', () => cb());
