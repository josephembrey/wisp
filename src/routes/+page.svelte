<script lang="ts">
	import { resizeWindow as resizeWindowCmd } from '$lib/tauri';
	import { app } from '$lib/state.svelte';
	import { onMount } from 'svelte';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import Titlebar from '$lib/components/titlebar.svelte';
	import TabGeneral from '$lib/components/tabs/general.svelte';
	import TabModel from '$lib/components/tabs/model.svelte';
	import TabOverlay from '$lib/components/tabs/overlay.svelte';
	import TabAbout from '$lib/components/tabs/about.svelte';
	import TabTranscribe from '$lib/components/tabs/transcribe.svelte';
	import TabHistory from '$lib/components/tabs/history.svelte';

	let contentHeight: number = $state(0);
	let tabHeight: number = $state(0);
	let tabAnimated: boolean = $state(false);
	let lastHeight = 0;

	const OUTER_PAD = 16;

	$effect(() => {
		const h = Math.ceil(contentHeight) + OUTER_PAD;
		if (h > 0 && Math.abs(h - lastHeight) >= 2) {
			lastHeight = h;
			resizeWindowCmd(h);
		}
	});

	$effect(() => {
		if (tabHeight > 0) {
			requestAnimationFrame(() => {
				tabAnimated = true;
			});
		}
	});

	onMount(() => app.init());
</script>

<div class="p-2">
	<div
		bind:clientHeight={contentHeight}
		class="overflow-hidden rounded-xl border border-border bg-card shadow-md"
	>
		<Titlebar />

		{#if app.settings}
			<Tabs.Root bind:value={app.activeTab}>
				<div class="px-3 pb-2">
					<Tabs.List class="w-full">
						<Tabs.Trigger value="general">General</Tabs.Trigger>
						<Tabs.Trigger value="model">Model</Tabs.Trigger>
						<Tabs.Trigger value="overlay">Overlay</Tabs.Trigger>
						<Tabs.Trigger value="transcribe">Scribe</Tabs.Trigger>
						<Tabs.Trigger value="history">History</Tabs.Trigger>
						<Tabs.Trigger value="about">About</Tabs.Trigger>
					</Tabs.List>
				</div>

				<div
					class="overflow-hidden"
					class:transition-[height]={tabAnimated}
					class:duration-200={tabAnimated}
					class:ease-out={tabAnimated}
					style:height={tabHeight ? `${tabHeight}px` : 'auto'}
				>
					<div bind:clientHeight={tabHeight} class="px-3 pb-3">
						<Tabs.Content value="general">
							<TabGeneral />
						</Tabs.Content>

						<Tabs.Content value="model">
							<TabModel />
						</Tabs.Content>

						<Tabs.Content value="overlay">
							<TabOverlay />
						</Tabs.Content>

						<Tabs.Content value="transcribe">
							<TabTranscribe />
						</Tabs.Content>

						<Tabs.Content value="history">
							<TabHistory />
						</Tabs.Content>

						<Tabs.Content value="about">
							<TabAbout />
						</Tabs.Content>
					</div>
				</div>
			</Tabs.Root>
		{:else}
			<div class="flex items-center justify-center p-8">
				<span class="text-xs text-muted-foreground">Loading...</span>
			</div>
		{/if}
	</div>
</div>
