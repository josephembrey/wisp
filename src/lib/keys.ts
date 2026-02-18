/** Map browser KeyboardEvent.code to the key names used by the hotkey system. */
export function mapBrowserKey(code: string): string {
	const map: Record<string, string> = {
		AltLeft: 'Alt',
		AltRight: 'RightAlt',
		ControlLeft: 'ControlLeft',
		ControlRight: 'ControlRight',
		ShiftLeft: 'ShiftLeft',
		ShiftRight: 'ShiftRight',
		MetaLeft: 'MetaLeft',
		MetaRight: 'MetaRight',
		Space: 'Space',
		CapsLock: 'CapsLock'
	};
	if (map[code]) return map[code];
	if (code.startsWith('Key')) return code;
	if (code.startsWith('Digit')) return 'Num' + code.slice(5);
	if (code.startsWith('F') && !isNaN(Number(code.slice(1)))) return code;
	return code;
}
