<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';

	let {
		active,
		onnavigate
	}: {
		active: string;
		onnavigate: (section: string) => void;
	} = $props();

	const sections = [
		{ id: 'main', label: 'Main' },
		{ id: 'advanced', label: 'Advanced' },
		{ id: 'overlay', label: 'Overlay' },
		{ id: 'about', label: 'About' }
	];
</script>

<nav class="flex flex-col gap-1">
	{#each sections as section (section.id)}
		<Button
			variant={active === section.id ? 'secondary' : 'ghost'}
			class="justify-start"
			onclick={() => onnavigate(section.id)}
		>
			{section.label}
		</Button>
	{/each}
</nav>
