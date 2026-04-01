import { onOverlayState, type OverlayState, type OverlayStatus } from '$lib/tauri';

const IDLE: OverlayState = { status: 'idle', ttl_ms: null };

// Overlay state — always idles unless an active state is set.
// States with a TTL auto-clear; states without persist until replaced or cleared.
let active: OverlayState | null = $state(null);
let timeout: ReturnType<typeof setTimeout> | undefined;

function push(s: OverlayState) {
	clearTimeout(timeout);
	if (s.status === 'idle') {
		active = null;
	} else {
		active = s;
		if (s.ttl_ms != null) {
			timeout = setTimeout(() => (active = null), s.ttl_ms);
		}
	}
}

function notify(status: OverlayStatus, ttl_ms: number) {
	push({ status, ttl_ms });
}

// Subscribe to backend overlay events (app-lifetime, never cleaned up)
onOverlayState((s) => push(s));

export const overlay = {
	get current(): OverlayState {
		return active ?? IDLE;
	},
	push,
	notify
};
