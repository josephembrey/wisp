<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Kbd } from '$lib/components/ui/kbd/index.js';
	import { SvelteSet } from 'svelte/reactivity';
	import XIcon from '@lucide/svelte/icons/x';

	let { hotkey, onsave }: { hotkey: string; onsave: (combo: string) => void } = $props();

	let capturing = $state(false);
	let capturedKeys = new SvelteSet<string>();

	// Map browser KeyboardEvent.code directly to Tauri accelerator format
	function toAccelerator(code: string): string | null {
		if (code.startsWith('Alt')) return 'Alt';
		if (code.startsWith('Control')) return 'Control';
		if (code.startsWith('Shift')) return 'Shift';
		if (code.startsWith('Meta')) return 'Super';
		if (code === 'Space') return 'Space';
		if (code === 'CapsLock') return 'CapsLock';
		if (code.startsWith('Key')) return code.slice(3); // KeyQ → Q
		if (code.startsWith('Digit')) return code.slice(5); // Digit5 → 5
		if (code.startsWith('F') && !isNaN(Number(code.slice(1)))) return code; // F12
		return null;
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
		const key = toAccelerator(e.code);
		if (key) capturedKeys.add(key);
	}

	function handleKeyup() {
		if (!capturing || capturedKeys.size === 0) return;
		const combo = Array.from(capturedKeys).join('+');
		capturing = false;
		onsave(combo);
	}
</script>

<svelte:window onkeydown={handleKeydown} onkeyup={handleKeyup} />

<div class="flex items-center gap-2">
	<!-- Capturing: show pressed keys with pulse animation -->
	{#if capturing}
		<Kbd class="animate-pulse px-2 py-1 text-sm">
			{capturedKeys.size > 0 ? Array.from(capturedKeys).join(' + ') : 'Press keys...'}
		</Kbd>
		<span class="text-xs text-muted-foreground">Release to save, Esc to cancel</span>

		<!-- Has hotkey: show current combo with change/clear buttons -->
	{:else if hotkey}
		<div class="flex flex-wrap items-center gap-1">
			{#each hotkey.split('+') as key, i (key)}
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
			<XIcon size={14} />
		</Button>

		<!-- No hotkey: show set button -->
	{:else}
		<span class="text-sm text-muted-foreground">None</span>
		<Button size="sm" variant="outline" onclick={startCapture} class="ml-auto">Set</Button>
	{/if}
</div>
