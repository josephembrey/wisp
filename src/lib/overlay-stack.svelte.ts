import type { OverlayState } from '$lib/tauri';

const IDLE: OverlayState = { icon: 'dot', label: 'Idle', ttl_ms: null };

export function createOverlayStack() {
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

	return {
		get current(): OverlayState {
			return transient ?? base;
		},
		push
	};
}
