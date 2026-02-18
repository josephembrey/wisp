<script lang="ts">
	import {
		getSettings,
		updateSettings,
		getStatus,
		getModels,
		downloadModel,
		deleteModel,
		onStatusChanged,
		onDownloadProgress,
		onTranscription,
		type Settings,
		type ModelInfo,
		type Status,
		type DownloadProgress
	} from '$lib/tauri';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';
	import { Kbd } from '$lib/components/ui/kbd/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { toggleMode, mode } from 'mode-watcher';
	import { SvelteSet } from 'svelte/reactivity';

	let settings: Settings | null = $state(null);
	let status: Status = $state('idle');
	let models: ModelInfo[] = $state([]);
	let downloading: string | null = $state(null);
	let progress: DownloadProgress | null = $state(null);
	let lastTranscription: string = $state('');

	// Hotkey capture state
	let capturing = $state(false);
	let capturedKeys = new SvelteSet<string>();

	const statusColor: Record<Status, string> = {
		idle: 'default',
		recording: 'destructive',
		processing: 'secondary'
	};

	const statusLabel: Record<Status, string> = {
		idle: 'Idle',
		recording: 'Recording',
		processing: 'Processing'
	};

	function mapBrowserKey(code: string): string {
		const map: Record<string, string> = {
			AltLeft: 'Alt',
			AltRight: 'RightAlt',
			ControlLeft: 'ControlLeft',
			ControlRight: 'ControlRight',
			ShiftLeft: 'ShiftLeft',
			ShiftRight: 'ShiftRight',
			MetaLeft: 'MetaLeft',
			MetaRight: 'MetaRight',
			Space: 'Space',
			CapsLock: 'CapsLock'
		};
		if (map[code]) return map[code];
		if (code.startsWith('Key')) return code;
		if (code.startsWith('Digit')) return 'Num' + code.slice(5);
		if (code.startsWith('F') && !isNaN(Number(code.slice(1)))) return code;
		return code;
	}

	function startCapture() {
		capturing = true;
		capturedKeys.clear();
	}

	function handleCaptureKeydown(e: KeyboardEvent) {
		if (!capturing) return;
		e.preventDefault();
		if (e.code === 'Escape') {
			capturing = false;
			return;
		}
		capturedKeys.add(mapBrowserKey(e.code));
	}

	function handleCaptureKeyup(_e: KeyboardEvent) {
		if (!capturing || capturedKeys.size === 0) return;
		const combo = Array.from(capturedKeys).join('+');
		capturing = false;
		save({ hotkey: combo });
	}

	async function load() {
		settings = await getSettings();
		status = await getStatus();
		models = await getModels();
	}

	async function save(updates: Partial<Settings>) {
		if (!settings) return;
		settings = { ...settings, ...updates };
		await updateSettings(settings);
	}

	async function handleDownload(name: string) {
		downloading = name;
		progress = null;
		try {
			await downloadModel(name);
			models = await getModels();
		} finally {
			downloading = null;
			progress = null;
		}
	}

	async function handleDelete(name: string) {
		await deleteModel(name);
		models = await getModels();
	}

	$effect(() => {
		load();

		const unsubs = [
			onStatusChanged((s) => (status = s)),
			onDownloadProgress((p) => (progress = p)),
			onTranscription((t) => (lastTranscription = t))
		];

		return () => {
			unsubs.forEach((p) => p.then((fn) => fn()));
		};
	});
</script>

<svelte:window onkeydown={handleCaptureKeydown} onkeyup={handleCaptureKeyup} />

<div class="flex h-screen flex-col">
	<!-- Custom titlebar -->
	<div
		class="flex h-9 shrink-0 items-center justify-between border-b border-border bg-card px-3"
		data-tauri-drag-region
	>
		<span class="pointer-events-none text-sm font-medium select-none" data-tauri-drag-region>
			Wisp
		</span>
		<div class="flex items-center gap-1">
			<button
				class="inline-flex h-6 w-6 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
				onclick={() => toggleMode()}
				aria-label="Toggle theme"
			>
				{#if mode.current === 'dark'}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><circle cx="12" cy="12" r="5" /><line x1="12" y1="1" x2="12" y2="3" /><line
							x1="12"
							y1="21"
							x2="12"
							y2="23"
						/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64" /><line
							x1="18.36"
							y1="18.36"
							x2="19.78"
							y2="19.78"
						/><line x1="1" y1="12" x2="3" y2="12" /><line x1="21" y1="12" x2="23" y2="12" /><line
							x1="4.22"
							y1="19.78"
							x2="5.64"
							y2="18.36"
						/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" /></svg
					>
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" /></svg
					>
				{/if}
			</button>
			<button
				class="inline-flex h-6 w-6 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
				onclick={() => getCurrentWindow().minimize()}
				aria-label="Minimize"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12" /></svg
				>
			</button>
			<button
				class="inline-flex h-6 w-6 items-center justify-center rounded-sm text-muted-foreground hover:bg-destructive hover:text-white"
				onclick={() => getCurrentWindow().hide()}
				aria-label="Close"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
				>
			</button>
		</div>
	</div>

	<!-- Scrollable content -->
	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto flex max-w-md flex-col gap-6 p-6">
			<div class="flex items-center justify-between">
				<h1 class="text-2xl font-semibold">Wisp</h1>
				<Badge variant={statusColor[status] as 'default' | 'destructive' | 'secondary'}>
					{statusLabel[status]}
				</Badge>
			</div>

			<Separator />

			{#if settings}
				<Card.Root>
					<Card.Header>
						<Card.Title>Model</Card.Title>
					</Card.Header>
					<Card.Content class="flex flex-col gap-3">
						<Select.Root
							type="single"
							value={settings.model}
							onValueChange={(v) => {
								if (v) save({ model: v });
							}}
						>
							<Select.Trigger class="w-full">
								{models.find((m) => m.name === settings?.model)?.name ?? settings.model}
							</Select.Trigger>
							<Select.Content>
								{#each models as model (model.name)}
									<Select.Item value={model.name}>
										{model.name}
										<span class="ml-auto text-xs text-muted-foreground">
											{model.size_mb} MB
											{#if model.downloaded}
												&check;
											{/if}
										</span>
									</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>

						{@const selectedModel = models.find((m) => m.name === settings?.model)}
						{#if selectedModel}
							<div class="flex gap-2">
								{#if !selectedModel.downloaded}
									<Button
										size="sm"
										onclick={() => handleDownload(selectedModel.name)}
										disabled={downloading !== null}
									>
										{downloading === selectedModel.name ? 'Downloading...' : 'Download'}
									</Button>
								{:else}
									<Button
										size="sm"
										variant="destructive"
										onclick={() => handleDelete(selectedModel.name)}
									>
										Delete
									</Button>
								{/if}
							</div>
						{/if}

						{#if downloading && progress && progress.total > 0}
							{@const pct = Math.round((progress.downloaded / progress.total) * 100)}
							<div class="flex flex-col gap-1.5">
								<Progress value={pct} />
								<span class="text-xs text-muted-foreground">{pct}%</span>
							</div>
						{/if}
					</Card.Content>
				</Card.Root>

				<Card.Root>
					<Card.Header>
						<Card.Title>Output</Card.Title>
					</Card.Header>
					<Card.Content>
						<div class="flex items-center justify-between">
							<Label for="output-mode">
								{settings.output_mode === 'clipboard' ? 'Copy to clipboard' : 'Type at cursor'}
							</Label>
							<Switch
								id="output-mode"
								checked={settings.output_mode === 'paste'}
								onCheckedChange={(checked) =>
									save({ output_mode: checked ? 'paste' : 'clipboard' })}
							/>
						</div>
						<p class="mt-1.5 text-xs text-muted-foreground">
							{settings.output_mode === 'clipboard'
								? 'Transcribed text is copied to your clipboard.'
								: 'Transcribed text is typed directly at your cursor.'}
						</p>
					</Card.Content>
				</Card.Root>

				<Card.Root>
					<Card.Header>
						<Card.Title>Hotkey</Card.Title>
					</Card.Header>
					<Card.Content class="flex flex-col gap-3">
						<div class="flex items-center gap-3">
							{#if capturing}
								<Kbd class="animate-pulse">
									{capturedKeys.size > 0 ? Array.from(capturedKeys).join(' + ') : 'Press keys...'}
								</Kbd>
								<span class="text-sm text-muted-foreground">Release to save, Esc to cancel</span>
							{:else}
								<Kbd>{settings.hotkey.replaceAll('+', ' + ')}</Kbd>
								<span class="text-sm text-muted-foreground">Hold to record</span>
							{/if}
						</div>
						<Button size="sm" variant="outline" onclick={startCapture} disabled={capturing}>
							Change hotkey
						</Button>
					</Card.Content>
				</Card.Root>

				{#if lastTranscription}
					<Card.Root>
						<Card.Header>
							<Card.Title>Last Transcription</Card.Title>
						</Card.Header>
						<Card.Content>
							<p class="text-sm">{lastTranscription}</p>
						</Card.Content>
					</Card.Root>
				{/if}
			{/if}
		</div>
	</div>
</div>
