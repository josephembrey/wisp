<script lang="ts">
	import type { Snippet } from 'svelte';

	let { label, description, children }: { label: string; description?: string; children: Snippet } = $props();
</script>

<div class="flex flex-col gap-1.5">
	<span class="text-xs font-medium text-muted-foreground">{label}{#if description}<span class="font-normal text-muted-foreground/60"> — {description}</span>{/if}</span>
	<div>
		{@render children()}
	</div>
</div>
