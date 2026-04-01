<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Kbd } from '$lib/components/ui/kbd/index.js';
	import { getVersion } from '@tauri-apps/api/app';
	import { resetApp, showLogDir, checkForUpdate, openUrl } from '$lib/tauri';
	import { app } from '$lib/state.svelte';
	import { onMount } from 'svelte';
	import DownloadIcon from '@lucide/svelte/icons/download';
	import KeyboardIcon from '@lucide/svelte/icons/keyboard';
	import ChevronRightIcon from '@lucide/svelte/icons/chevron-right';
	import RefreshCwIcon from '@lucide/svelte/icons/refresh-cw';

	const isDev = import.meta.env.DEV;
	const card =
		'flex items-center gap-3 rounded-lg border border-border p-3 text-left transition-colors hover:border-muted-foreground/50';
	const cardIcon =
		'flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground';
	const panel = 'flex flex-col gap-2 rounded-lg border border-border p-3';

	let version = $state('');
	let checking = $state(false);

	onMount(() => {
		getVersion().then((v) => (version = v));
	});

	async function manualCheck() {
		checking = true;
		try {
			const info = await checkForUpdate();
			app.updateInfo = info;
		} finally {
			checking = false;
		}
	}
</script>

<div class="flex flex-col gap-3">
	<!-- Tagline -->
	<p class="text-xs text-muted-foreground">
		Local push-to-talk dictation. Everything stays on your machine.
	</p>

	<!-- Action cards -->
	<button class={card} onclick={() => (app.activeTab = 'model')}>
		<div class={cardIcon}><DownloadIcon size={14} /></div>
		<div class="flex-1">
			<div class="text-xs font-medium text-foreground">Download a model</div>
			<div class="text-[10px] text-muted-foreground/60">Pick a Whisper model to get started</div>
		</div>
		<ChevronRightIcon size={14} class="text-muted-foreground/40" />
	</button>

	<button class={card} onclick={() => (app.activeTab = 'general')}>
		<div class={cardIcon}><KeyboardIcon size={14} /></div>
		<div class="flex-1">
			<div class="text-xs font-medium text-foreground">Your hotkey</div>
			<div class="text-[10px] text-muted-foreground/60">Hold to record, release to transcribe</div>
		</div>
		<Kbd class="px-2 py-1 text-sm">
			{(app.settings?.hotkey ?? '').replace(/\+/g, ' + ')}
		</Kbd>
	</button>

	<!-- Tips + Updates row -->
	<div class="flex gap-2">
		<!-- Tips panel -->
		<div class="{panel} flex-1">
			<ul class="space-y-1 text-xs text-muted-foreground">
				{#each ['Larger models are slower but more accurate', 'Enable GPU for faster transcription', 'Runs in the tray when minimized'] as tip (tip)}
					<li class="flex items-start gap-2">
						<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground/40"></span>
						<span>{tip}</span>
					</li>
				{/each}
			</ul>
		</div>

		<!-- Updates panel -->
		<div class="{panel} w-[150px] shrink-0 items-center justify-center text-center">
			<Button
				size="sm"
				variant="outline"
				class="h-6 w-full gap-1.5 text-[10px]"
				onclick={manualCheck}
				disabled={checking}
			>
				<RefreshCwIcon size={10} class={checking ? 'animate-spin' : ''} />
				{checking ? 'Checking' : 'Check for updates'}
			</Button>
			<span class="h-4 text-[10px]">
				{#if app.updateInfo?.available}
					<button
						class="text-primary underline hover:text-primary/80"
						onclick={() => openUrl(app.updateInfo!.url)}>v{app.updateInfo.latest} available</button
					>
				{:else if app.updateInfo}
					<span class="text-muted-foreground/60">Up to date</span>
				{:else}
					<span class="text-muted-foreground/40">Not checked</span>
				{/if}
			</span>
		</div>
	</div>

	<!-- Reset + version -->
	<div class="flex items-center gap-3">
		<AlertDialog.Root>
			<AlertDialog.Trigger
				class="text-[10px] text-muted-foreground/40 underline hover:text-muted-foreground"
			>
				Reset app
			</AlertDialog.Trigger>
			<AlertDialog.Content>
				<AlertDialog.Header>
					<AlertDialog.Title>Reset Wisp?</AlertDialog.Title>
					<AlertDialog.Description>
						This will delete all settings and downloaded models, then restart the app.
					</AlertDialog.Description>
				</AlertDialog.Header>
				<AlertDialog.Footer>
					<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action onclick={() => resetApp()}>Reset</AlertDialog.Action>
				</AlertDialog.Footer>
			</AlertDialog.Content>
		</AlertDialog.Root>

		{#if isDev}
			<button
				class="text-[10px] text-muted-foreground/40 underline hover:text-muted-foreground"
				onclick={() => showLogDir()}
			>
				Show logs
			</button>
		{/if}
		{#if version}<span class="ml-auto text-[10px] text-muted-foreground/40">v{version}</span>{/if}
	</div>
</div>
