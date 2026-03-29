<script lang="ts">
	import { onMount } from 'svelte';
	import { resizeWindow as resizeWindowCmd } from '$lib/tauri';
	import { app } from '$lib/state.svelte';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import * as View from '$lib/components/views';
	import Titlebar from '$lib/components/titlebar.svelte';

	let contentHeight: number = $state(0);
	let tabHeight: number = $state(0);
	let lastHeight = 0;

	const OUTER_PAD = 16;
	$effect(() => {
		const h = Math.ceil(contentHeight) + OUTER_PAD;
		if (h > 0 && Math.abs(h - lastHeight) >= 2) {
			lastHeight = h;
			resizeWindowCmd(h);
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
					class="overflow-hidden transition-[height] duration-200 ease-out"
					style:height={tabHeight ? `${tabHeight}px` : 'auto'}
				>
					<div bind:clientHeight={tabHeight} class="px-3 pb-3">
						<Tabs.Content value="general">
							<View.General />
						</Tabs.Content>

						<Tabs.Content value="model">
							<View.Model />
						</Tabs.Content>

						<Tabs.Content value="overlay">
							<View.Overlay />
						</Tabs.Content>

						<Tabs.Content value="transcribe">
							<View.Transcribe />
						</Tabs.Content>

						<Tabs.Content value="history">
							<View.History />
						</Tabs.Content>

						<Tabs.Content value="about">
							<View.About />
						</Tabs.Content>
					</div>
				</div>
			</Tabs.Root>
		{:else}
			<div class="flex items-center justify-center p-8">
				<Loader2Icon size={16} class="animate-spin text-muted-foreground" />
			</div>
		{/if}
	</div>
</div>
