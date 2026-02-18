<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Kbd } from '$lib/components/ui/kbd/index.js';
	import { SvelteSet } from 'svelte/reactivity';

	let { hotkey, onsave }: { hotkey: string; onsave: (combo: string) => void } = $props();

	let capturing = $state(false);
	let capturedKeys = new SvelteSet<string>();

	function mapBrowserKey(code: string): string {
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

	function startCapture() {
		capturing = true;
		capturedKeys.clear();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!capturing) return;
		e.preventDefault();
		if (e.code === 'Escape') {
			capturing = false;
			return;
		}
		capturedKeys.add(mapBrowserKey(e.code));
	}

	function handleKeyup(_e: KeyboardEvent) {
		if (!capturing || capturedKeys.size === 0) return;
		const combo = Array.from(capturedKeys).join('+');
		capturing = false;
		onsave(combo);
	}
</script>

<svelte:window onkeydown={handleKeydown} onkeyup={handleKeyup} />

<div class="flex items-center gap-2">
	{#if capturing}
		<Kbd class="animate-pulse px-2 py-1 text-sm">
			{capturedKeys.size > 0 ? Array.from(capturedKeys).join(' + ') : 'Press keys...'}
		</Kbd>
		<span class="text-xs text-muted-foreground">Release to save, Esc to cancel</span>
	{:else if hotkey}
		<div class="flex flex-wrap items-center gap-1">
			{#each hotkey.split('+') as key, i}
				{#if i > 0}
					<span class="text-xs text-muted-foreground">+</span>
				{/if}
				<Kbd class="px-2 py-1 text-sm">{key}</Kbd>
			{/each}
		</div>
		<Button size="sm" variant="outline" onclick={startCapture} class="ml-auto">Change</Button>
		<Button
			size="sm"
			variant="ghost"
			onclick={() => onsave('')}
			class="h-8 w-8 shrink-0 p-0"
			aria-label="Clear hotkey"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
			>
		</Button>
	{:else}
		<span class="text-sm text-muted-foreground">None</span>
		<Button size="sm" variant="outline" onclick={startCapture} class="ml-auto">Set</Button>
	{/if}
</div>
