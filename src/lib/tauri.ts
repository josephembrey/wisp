import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { commands, type Result } from './bindings';

export type {
	Settings,
	OverlayState,
	OverlayStatus,
	ModelInfo,
	InputDeviceInfo,
	MonitorInfo,
	MemoryInfo,
	DownloadProgress,
	OutputMode,
	ModelLoading,
	HistoryEntry
} from './bindings';

// Result-unwrapping helpers for commands that return Result<T, string>
function unwrap<T>(result: Result<T, string>): T {
	if (result.status === 'error') throw new Error(result.error);
	return result.data;
}

// Commands (pass-through for simple getters, unwrap for fallible actions)
export const isFirstRun = commands.isFirstRun;
export const getSettings = commands.getSettings;
export const getModels = commands.getModels;
export const getGpuBackend = commands.getGpuBackend;
export const getMonitors = commands.getMonitors;
export const getMemoryInfo = commands.getMemoryInfo;
export const getInputDevices = commands.getInputDevices;
export const resizeWindow = commands.resizeWindow;
export const quit = commands.quit;

export const updateSettings = async (settings: import('./bindings').Settings) =>
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

// Event listeners
export const onOverlayState = (
	cb: (state: import('./bindings').OverlayState) => void
): Promise<UnlistenFn> =>
	listen<import('./bindings').OverlayState>('overlay-state', (e) => cb(e.payload));

export const onDownloadProgress = (
	cb: (progress: import('./bindings').DownloadProgress) => void
): Promise<UnlistenFn> =>
	listen<import('./bindings').DownloadProgress>('download-progress', (e) => cb(e.payload));

export const onTranscription = (cb: (text: string) => void): Promise<UnlistenFn> =>
	listen<string>('transcription', (e) => cb(e.payload));

export const onError = (cb: (message: string) => void): Promise<UnlistenFn> =>
	listen<string>('backend-error', (e) => cb(e.payload));

export const onSettingsChanged = (
	cb: (settings: import('./bindings').Settings) => void
): Promise<UnlistenFn> =>
	listen<import('./bindings').Settings>('settings-changed', (e) => cb(e.payload));

export const onTranscribeFileProgress = (cb: (status: string) => void): Promise<UnlistenFn> =>
	listen<string>('transcribe-file-progress', (e) => cb(e.payload));

export const onHistoryChanged = (cb: () => void): Promise<UnlistenFn> =>
	listen('history-changed', () => cb());
