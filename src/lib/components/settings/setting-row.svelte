<script lang="ts">
	import type { Snippet } from 'svelte';

	let { label, children }: { label: string; children: Snippet } = $props();
</script>

<div class="flex flex-col gap-1.5">
	<span class="text-xs font-medium text-muted-foreground">{label}</span>
	<div>
		{@render children()}
	</div>
</div>
