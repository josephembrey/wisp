import { info, warn, error } from '@tauri-apps/plugin-log';

export const log = {
	info: (msg: string) => { info(msg); },
	warn: (msg: string) => { warn(msg); },
	error: (msg: string) => { error(msg); }
};
