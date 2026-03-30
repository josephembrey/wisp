<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { resizeWindow as resizeWindowCmd } from '$lib/tauri';
	import { app } from '$lib/state.svelte';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import * as View from '$lib/components/views';
	import Titlebar from '$lib/components/titlebar.svelte';

	// Window resize flow:
	// 1. Tab content swaps instantly (clipped by overflow:hidden)
	// 2. Measure new content height via calcWindowHeight (header + inner + pad)
	// 3. Growing: await resizeWindowCmd → animate card height
	// 4. Shrinking: animate card height → transitionend → resizeWindowCmd
	let cardEl: HTMLDivElement | undefined = $state();
	let headerEl: HTMLDivElement | undefined = $state();
	let innerEl: HTMLDivElement | undefined = $state();
	let tabHeight: number = $state(0);
	let lastWindowHeight = 0;
	let animating = $state(false);

	const OUTER_PAD = 8;

	function calcWindowHeight(innerHeight: number): number {
		const headerHeight = headerEl ? headerEl.getBoundingClientRect().height : 0;
		const cardBorder = cardEl ? cardEl.offsetHeight - cardEl.clientHeight : 0;
		return Math.ceil(headerHeight + innerHeight + cardBorder + OUTER_PAD);
	}

	async function onTabChange(newTab: string) {
		// 1. Swap content immediately (no animation)
		animating = false;
		app.activeTab = newTab;
		await tick();

		if (!innerEl) return;

		// 2. Measure new content
		const targetInner = innerEl.scrollHeight;
		const targetWindow = calcWindowHeight(targetInner);
		const growing = targetWindow > lastWindowHeight;

		// Growing: resize window first to make room
		if (growing) {
			lastWindowHeight = targetWindow;
			await resizeWindowCmd(targetWindow);
		}
		// Animate height change (shrink resizes on transitionend)
		if (growing || targetWindow < lastWindowHeight) {
			animating = true;
			await tick();
		}
		tabHeight = targetInner;
	}

	function onTransitionEnd(e: TransitionEvent) {
		if (e.propertyName !== 'height') return;
		animating = false;
		const h = calcWindowHeight(tabHeight);
		if (h !== lastWindowHeight) {
			lastWindowHeight = h;
			resizeWindowCmd(h);
		}
	}

	// Resize window when content height changes (initial sizing + within-tab changes)
	$effect(() => {
		if (!innerEl) return;
		const observer = new ResizeObserver(() => {
			if (animating) return;
			const targetInner = innerEl.scrollHeight;
			const targetWindow = calcWindowHeight(targetInner);
			if (targetWindow !== lastWindowHeight) {
				lastWindowHeight = targetWindow;
				tabHeight = targetInner;
				resizeWindowCmd(targetWindow);
			}
		});
		observer.observe(innerEl);
		return () => observer.disconnect();
	});

	onMount(() => app.init());
</script>

<!-- Settings window -->
<div class="p-1">
	<div bind:this={cardEl} class="overflow-hidden rounded-xl border border-border bg-card shadow-sm">
		{#if app.settings}
			<Tabs.Root
				class="gap-0"
				value={app.activeTab}
				onValueChange={(v) => {
					if (v) onTabChange(v);
				}}
			>
				<!-- Header: titlebar + tab bar (measured for height calc) -->
				<div bind:this={headerEl}>
					<Titlebar />
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
				</div>

				<!-- Tab content (animated height) -->
				<div
					class="overflow-hidden {animating ? 'transition-[height] duration-200 ease-out' : ''}"
					style:height={tabHeight ? `${tabHeight}px` : 'auto'}
					ontransitionend={onTransitionEnd}
				>
					<div bind:this={innerEl} class="flow-root px-3 pb-3">
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

			<!-- Loading state -->
		{:else}
			<Titlebar />
			<div class="flex items-center justify-center p-8">
				<Loader2Icon size={16} class="animate-spin text-muted-foreground" />
			</div>
		{/if}
	</div>
</div>
