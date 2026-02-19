<script lang="ts">
	import { Separator } from '$lib/components/ui/separator/index.js';

	let { hotkey }: { hotkey: string } = $props();
</script>

<div class="flex flex-col gap-4">
	<p class="text-sm leading-relaxed text-muted-foreground">
		Wisp is a push-to-talk dictation app. Hold a hotkey to record, release to transcribe locally
		with Whisper, and the text goes to your clipboard or cursor.
	</p>

	<Separator />

	<div class="flex flex-col gap-2">
		<span class="text-xs font-semibold tracking-wide text-foreground uppercase">Quick Start</span>
		<ol class="space-y-2 text-xs text-muted-foreground">
			<li class="flex gap-2">
				<span class="font-semibold text-foreground">1.</span>
				<span>Download a model in <strong>Advanced</strong> (base is a good start)</span>
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

	<Separator />

	<div class="flex flex-col gap-2">
		<span class="text-xs font-semibold tracking-wide text-foreground uppercase">Tips</span>
		<ul class="space-y-1.5 text-xs text-muted-foreground">
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
				<span>Enable GPU in Advanced for faster transcription</span>
			</li>
		</ul>
	</div>
</div>
