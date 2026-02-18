<script lang="ts">
	import type { Snippet } from 'svelte';

	let { label, children }: { label: string; children: Snippet } = $props();
</script>

<div class="grid grid-cols-[5rem_1fr] items-center gap-2">
	<label class="text-sm font-medium">{label}</label>
	<div>
		{@render children()}
	</div>
</div>
