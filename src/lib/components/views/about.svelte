<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { Kbd } from '$lib/components/ui/kbd/index.js';
	import { resetApp, showLogDir } from '$lib/tauri';
	import { app } from '$lib/state.svelte';
	import DownloadIcon from '@lucide/svelte/icons/download';
	import KeyboardIcon from '@lucide/svelte/icons/keyboard';
	import ChevronRightIcon from '@lucide/svelte/icons/chevron-right';

	const isDev = import.meta.env.DEV;
	const card =
		'flex items-center gap-3 rounded-lg border border-border p-3 text-left transition-colors hover:border-muted-foreground/50';
	const cardIcon =
		'flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground';
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

	<!-- Tips -->
	<ul class="space-y-1 text-xs text-muted-foreground">
		{#each ['Larger models are slower but more accurate', 'Enable GPU in Model for faster transcription', 'Wisp keeps running in the tray when minimized'] as tip (tip)}
			<li class="flex items-start gap-2">
				<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground/40"></span>
				<span>{tip}</span>
			</li>
		{/each}
	</ul>

	<!-- Reset -->
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
	</div>
</div>
