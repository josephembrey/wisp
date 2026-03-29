<script lang="ts">
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { toast } from 'svelte-sonner';
	import { info, error as logError } from '@tauri-apps/plugin-log';
	import { transcribeFile, onTranscribeFileProgress } from '$lib/tauri';
	import Button from '$lib/components/ui/button/button.svelte';
	import { app } from '$lib/state.svelte';
	import UploadIcon from '@lucide/svelte/icons/upload';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';

	type FileStatus = 'idle' | 'decoding' | 'loading' | 'transcribing';

	let status: FileStatus = $state('idle');
	let result: string = $state('');
	let filePath: string = $state('');
	let dragOver: boolean = $state(false);
	let copied: boolean = $state(false);
	let copiedTimeout: ReturnType<typeof setTimeout> | undefined;

	const busy = $derived(status !== 'idle');

	const statusLabel: Record<FileStatus, string> = {
		idle: '',
		decoding: 'Decoding audio...',
		loading: 'Loading model...',
		transcribing: 'Transcribing...'
	};

	async function handleFile(path: string) {
		filePath = path;
		result = '';
		info(`[transcribe] file: ${path}`);
		try {
			const text = await transcribeFile(path);
			result = text;
			info(`[transcribe] done: ${text.length} chars`);
		} catch (e) {
			logError(`[transcribe] failed: ${e}`);
			toast.error(`Transcription failed: ${e}`);
		} finally {
			status = 'idle';
		}
	}

	async function pickFile() {
		const selected = await open({
			multiple: false,
			filters: [
				{
					name: 'Audio',
					extensions: ['wav', 'mp3', 'flac', 'ogg', 'oga', 'm4a', 'aac', 'wma', 'opus', 'webm']
				}
			]
		});
		if (selected) {
			handleFile(selected);
		}
	}

	async function copyResult() {
		await navigator.clipboard.writeText(result);
		clearTimeout(copiedTimeout);
		copied = true;
		copiedTimeout = setTimeout(() => (copied = false), 1500);
	}

	function fileName(path: string) {
		return path.split(/[\\/]/).pop() ?? path;
	}

	onMount(() => {
		const unsubs = [
			onTranscribeFileProgress((s) => {
				if (s === 'done') {
					status = 'idle';
				} else {
					status = s as FileStatus;
				}
			})
		];

		const webview = getCurrentWebviewWindow();
		const dropPromise = webview.onDragDropEvent((event) => {
			if (busy) return;
			if (event.payload.type === 'over') {
				dragOver = true;
			} else if (event.payload.type === 'leave') {
				dragOver = false;
			} else if (event.payload.type === 'drop') {
				dragOver = false;
				const paths = event.payload.paths;
				if (paths.length > 0) {
					handleFile(paths[0]);
				}
			}
		});

		return () => {
			unsubs.forEach((p) => p.then((fn) => fn()));
			dropPromise.then((fn) => fn());
		};
	});
</script>

<div class="flex flex-col gap-3">
	<!-- Drop zone -->
	<button
		class="flex flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed p-6 transition-colors
			{dragOver
			? 'border-primary bg-primary/5'
			: 'border-muted-foreground/25 hover:border-muted-foreground/50'}
			{busy ? 'pointer-events-none opacity-50' : 'cursor-pointer'}"
		onclick={pickFile}
		disabled={busy}
	>
		{#if busy}
			<Loader2Icon size={32} class="animate-spin text-muted-foreground" />
			<span class="text-xs text-muted-foreground">{statusLabel[status]}</span>
		{:else}
			<UploadIcon size={32} class="text-muted-foreground/50" />
			<span class="text-xs text-muted-foreground">
				Drop an audio file or <span class="underline">browse</span>
			</span>
		{/if}
	</button>

	<!-- File name -->
	{#if filePath}
		<p class="truncate text-xs text-muted-foreground" title={filePath}>
			{fileName(filePath)}
		</p>
	{/if}

	<!-- Result -->
	{#if result}
		<div class="flex flex-col gap-1.5">
			<div class="flex items-center justify-between">
				<span class="text-xs font-semibold tracking-wide text-foreground uppercase">Result</span>
				<Button variant="ghost" size="sm" class="h-6 px-2 text-xs" onclick={copyResult}>
					{copied ? 'Copied' : 'Copy'}
				</Button>
			</div>
			<div
				class="max-h-40 overflow-y-auto rounded-md border bg-muted/50 p-2 text-xs leading-relaxed text-foreground select-text"
			>
				{result}
			</div>
		</div>
	{/if}

	<p class="text-xs text-muted-foreground">
		Using model <strong>{app.settings!.model}</strong> &middot; {app.settings!.language === 'auto'
			? 'auto-detect'
			: app.settings!.language}
	</p>
</div>
