import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { commands, type Result } from './bindings';

export type {
	Settings,
	Status,
	ModelInfo,
	InputDeviceInfo,
	MonitorInfo,
	DownloadProgress,
	OutputMode,
	ModelLoading
} from './bindings';

// Result-unwrapping helpers for commands that return Result<T, string>
function unwrap<T>(result: Result<T, string>): T {
	if (result.status === 'error') throw new Error(result.error);
	return result.data;
}

// Commands (pass-through for simple getters, unwrap for fallible actions)
export const isFirstRun = commands.isFirstRun;
export const getSettings = commands.getSettings;
export const getStatus = commands.getStatus;
export const getModels = commands.getModels;
export const getGpuBackend = commands.getGpuBackend;
export const getMonitors = commands.getMonitors;
export const getInputDevices = commands.getInputDevices;
export const resizeWindow = commands.resizeWindow;
export const quit = commands.quit;

export const updateSettings = async (settings: import('./bindings').Settings) =>
	unwrap(await commands.updateSettings(settings));
export const downloadModel = async (name: string) => unwrap(await commands.downloadModel(name));
export const deleteModel = async (name: string) => unwrap(await commands.deleteModel(name));
export const resetApp = async () => unwrap(await commands.resetApp());

// Window helpers
export const hideWindow = () => getCurrentWindow().hide();
export const minimizeWindow = () => getCurrentWindow().minimize();

// Event listeners
export const onStatusChanged = (
	cb: (status: import('./bindings').Status) => void
): Promise<UnlistenFn> => listen<import('./bindings').Status>('status-changed', (e) => cb(e.payload));

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

export const onOverlayFlash = (cb: (message: string) => void): Promise<UnlistenFn> =>
	listen<string>('overlay-flash', (e) => cb(e.payload));
