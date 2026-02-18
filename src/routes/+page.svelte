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

	const HOTKEYS = [
		'RightAlt',
		'Alt',
		'ControlLeft',
		'ControlRight',
		'ShiftRight',
		'MetaRight',
		'F1',
		'F2',
		'F3',
		'F4',
		'F5',
		'F6',
		'F7',
		'F8',
		'F9',
		'F10',
		'F11',
		'F12'
	];

	let settings: Settings | null = $state(null);
	let status: Status = $state('idle');
	let models: ModelInfo[] = $state([]);
	let downloading: string | null = $state(null);
	let progress: DownloadProgress | null = $state(null);
	let lastTranscription: string = $state('');

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
					<div class="flex flex-col gap-1.5">
						<Progress value={progress.downloaded} max={progress.total} />
						<span class="text-xs text-muted-foreground">
							{Math.round((progress.downloaded / progress.total) * 100)}%
						</span>
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
						onCheckedChange={(checked) => save({ output_mode: checked ? 'paste' : 'clipboard' })}
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
					<Kbd>{settings.hotkey}</Kbd>
					<span class="text-sm text-muted-foreground">Hold to record</span>
				</div>
				<Select.Root
					type="single"
					value={settings.hotkey}
					onValueChange={(v) => {
						if (v) save({ hotkey: v });
					}}
				>
					<Select.Trigger class="w-full">
						{settings.hotkey}
					</Select.Trigger>
					<Select.Content>
						{#each HOTKEYS as key (key)}
							<Select.Item value={key}>{key}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
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
