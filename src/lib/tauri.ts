import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export interface Settings {
	model: string;
	output_mode: 'clipboard' | 'paste';
	hotkey: string;
	language: string;
	gpu: boolean;
	interrupt: boolean;
	output_hotkey: string;
	min_duration: number;
}

export interface ModelInfo {
	name: string;
	size_mb: number;
	downloaded: boolean;
}

export interface DownloadProgress {
	model: string;
	downloaded: number;
	total: number;
}

export type Status = 'idle' | 'loading' | 'recording' | 'processing';

export const isFirstRun = () => invoke<boolean>('is_first_run');
export const getSettings = () => invoke<Settings>('get_settings');
export const updateSettings = (settings: Settings) => invoke('update_settings', { settings });
export const getStatus = () => invoke<Status>('get_status');
export const getModels = () => invoke<ModelInfo[]>('get_models');
export const downloadModel = (name: string) => invoke('download_model', { name });
export const deleteModel = (name: string) => invoke('delete_model', { name });
export const getGpuBackend = () => invoke<string>('get_gpu_backend');
export const resetApp = () => invoke('reset_app');
export const quit = () => invoke('quit');

export const onStatusChanged = (cb: (status: Status) => void): Promise<UnlistenFn> =>
	listen<Status>('status-changed', (e) => cb(e.payload));

export const onDownloadProgress = (cb: (progress: DownloadProgress) => void): Promise<UnlistenFn> =>
	listen<DownloadProgress>('download-progress', (e) => cb(e.payload));

export const onTranscription = (cb: (text: string) => void): Promise<UnlistenFn> =>
	listen<string>('transcription', (e) => cb(e.payload));

export const onError = (cb: (message: string) => void): Promise<UnlistenFn> =>
	listen<string>('backend-error', (e) => cb(e.payload));

export const onSettingsChanged = (cb: () => void): Promise<UnlistenFn> =>
	listen('settings-changed', () => cb());
