<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Switch } from '$lib/components/ui/switch/index.js';

	let {
		checked,
		label,
		onchange,
		children
	}: {
		checked: boolean;
		label?: string;
		onchange: (value: boolean) => void;
		children?: Snippet;
	} = $props();
</script>

<div
	class="flex cursor-pointer items-center gap-3"
	role="switch"
	tabindex="0"
	aria-checked={checked}
	onclick={() => onchange(!checked)}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			onchange(!checked);
		}
	}}
>
	<Switch {checked} class="pointer-events-none" />
	{#if children}
		{@render children()}
	{:else if label}
		<span class="text-xs text-muted-foreground">{label}</span>
	{/if}
</div>
