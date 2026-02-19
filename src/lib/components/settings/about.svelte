<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { resetApp } from '$lib/tauri';

	let {
		hotkey
	}: {
		hotkey: string;
	} = $props();
</script>

<div class="flex flex-col gap-3">
	<p class="text-xs leading-relaxed text-muted-foreground">
		Push-to-talk dictation. Hold a hotkey to record, release to transcribe locally with Whisper.
	</p>

	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-semibold tracking-wide text-foreground uppercase">Quick Start</span>
		<ol class="space-y-1 text-xs text-muted-foreground">
			<li class="flex gap-2">
				<span class="font-semibold text-foreground">1.</span>
				<span>Download a model in <strong>Model</strong> (base is a good start)</span>
			</li>
			<li class="flex gap-2">
				<span class="font-semibold text-foreground">2.</span>
				<span>Hold <strong>{hotkey.replace(/\+/g, ' + ')}</strong> to record</span>
			</li>
			<li class="flex gap-2">
				<span class="font-semibold text-foreground">3.</span>
				<span>Release to transcribe</span>
			</li>
		</ol>
	</div>

	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-semibold tracking-wide text-foreground uppercase">Tips</span>
		<ul class="space-y-1 text-xs text-muted-foreground">
			<li class="flex items-start gap-2">
				<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
				<span>Minimize this window — Wisp keeps running in the system tray</span>
			</li>
			<li class="flex items-start gap-2">
				<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
				<span
					><strong>Clipboard</strong> copies text, <strong>Type</strong> pastes at your cursor</span
				>
			</li>
			<li class="flex items-start gap-2">
				<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
				<span>Larger models are slower but more accurate</span>
			</li>
			<li class="flex items-start gap-2">
				<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
				<span>Enable GPU in Model for faster transcription</span>
			</li>
		</ul>
	</div>

	<AlertDialog.Root>
		<AlertDialog.Trigger class="text-xs text-muted-foreground underline hover:text-foreground">
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
</div>
