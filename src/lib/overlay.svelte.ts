import { onOverlayState, type OverlayState, type OverlayStatus } from '$lib/tauri';

const IDLE: OverlayState = { status: 'idle', label: 'Idle', ttl_ms: null };

// Overlay notification stack — shared between settings window and overlay window.
// Two-slot design: persistent base state + optional timed transient.
let base: OverlayState = $state(IDLE);
let transient: OverlayState | null = $state(null);
let timeout: ReturnType<typeof setTimeout> | undefined;

function push(s: OverlayState) {
	if (s.ttl_ms != null) {
		clearTimeout(timeout);
		transient = s;
		timeout = setTimeout(() => (transient = null), s.ttl_ms);
	} else {
		clearTimeout(timeout);
		base = s;
		transient = null;
	}
}

function notify(label: string, status: OverlayStatus, ttl_ms: number) {
	push({ status, label, ttl_ms });
}

// Subscribe to backend overlay events
onOverlayState((s) => push(s));

export const overlay = {
	get current(): OverlayState {
		return transient ?? base;
	},
	push,
	notify
};
