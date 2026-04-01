<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { SettingSwitch } from '$lib/components/ui/setting-switch/index.js';
	import { Slider } from '$lib/components/ui/slider/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';

	import { app } from '$lib/state.svelte';
	import type { ModelLoading } from '$lib/tauri';
	import Trash2Icon from '@lucide/svelte/icons/trash-2';

	let selectedModel = $derived(app.models.find((m) => m.name === app.settings!.model));
	const pct = (value: number, total: number) => (total ? (value / total) * 100 : 0);
	let memUsed = $derived(app.memoryInfo.total_mb - app.memoryInfo.available_mb);
	let memUsedPct = $derived(pct(memUsed, app.memoryInfo.total_mb));
	let memModelPct = $derived(pct(selectedModel?.size_mb ?? 0, app.memoryInfo.total_mb));

	const languages = [
		{ value: 'auto', label: 'Auto-detect' },
		{ value: 'en', label: 'English' },
		{ value: 'es', label: 'Spanish' },
		{ value: 'fr', label: 'French' },
		{ value: 'de', label: 'German' },
		{ value: 'it', label: 'Italian' },
		{ value: 'pt', label: 'Portuguese' },
		{ value: 'zh', label: 'Chinese' },
		{ value: 'ja', label: 'Japanese' },
		{ value: 'ko', label: 'Korean' },
		{ value: 'ru', label: 'Russian' },
		{ value: 'ar', label: 'Arabic' }
	];

	const formatMb = (mb: number) => (mb >= 1024 ? `${(mb / 1024).toFixed(1)} GB` : `${mb} MB`);

	const loadingModes = [
		{ value: 'eager', label: 'Startup' },
		{ value: 'lazy', label: 'First use' },
		{ value: 'per_use', label: 'Every use' }
	];
</script>

<div class="flex flex-col gap-3">
	<!-- Model selector + download/delete -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Model</span>
		<div class="flex items-center gap-2">
			<Select.Root
				type="single"
				value={app.settings!.model}
				onValueChange={(v) => {
					if (v) app.save({ model: v });
				}}
			>
				<Select.Trigger class="flex-1">
					{selectedModel?.name ?? app.settings!.model}
				</Select.Trigger>
				<Select.Content>
					{#each app.models as model (model.name)}
						<Select.Item value={model.name}>
							{model.name}
							<span class="ml-auto text-xs text-muted-foreground">
								{model.size_mb} MB
								{#if model.downloaded}&check;{/if}
							</span>
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>

			{#if selectedModel && !selectedModel.downloaded}
				<Button
					size="sm"
					onclick={() => app.downloadModel(selectedModel.name)}
					disabled={app.downloadProgress !== null}
				>
					{app.downloadProgress?.model === selectedModel.name ? 'Downloading...' : 'Download'}
				</Button>
			{:else if selectedModel}
				<Button
					size="sm"
					variant="outline"
					onclick={() => app.deleteModel(selectedModel.name)}
					class="h-8 w-8 shrink-0 p-0"
					aria-label="Delete model"
				>
					<Trash2Icon size={14} />
				</Button>
			{/if}
		</div>

		{#if app.downloadProgress && app.downloadProgress.total > 0}
			{@const pct = Math.round(
				(app.downloadProgress.downloaded / app.downloadProgress.total) * 100
			)}
			<div class="flex items-center gap-2">
				{#if app.downloadProgress.model !== selectedModel?.name}
					<span class="shrink-0 text-xs text-muted-foreground">{app.downloadProgress.model}</span>
				{/if}
				<Progress value={pct} class="flex-1" />
				<span class="text-xs text-muted-foreground tabular-nums">{pct}%</span>
			</div>
		{/if}
	</div>

	<!-- Memory bar -->
	{#if app.memoryInfo.total_mb}
		<div class="flex flex-col gap-1">
			<span class="text-xs font-medium text-muted-foreground"
				>{app.settings!.gpu ? 'GPU' : 'System'} Memory</span
			>
			<div class="flex h-2 w-full overflow-hidden rounded-full bg-muted">
				<div class="h-full bg-muted-foreground/30" style:width="{memUsedPct}%"></div>
				<div class="h-full bg-primary/50" style:width="{memModelPct}%"></div>
			</div>
			<div class="flex gap-3 text-[10px] text-muted-foreground">
				<span>{formatMb(memUsed)} used</span>
				<span>{formatMb(selectedModel?.size_mb ?? 0)} model</span>
				<span>{formatMb(app.memoryInfo.available_mb)} free</span>
			</div>
		</div>
	{/if}

	<!-- GPU -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground"
			>GPU Acceleration<span class="font-normal text-muted-foreground/60">
				— Use GPU for faster transcription</span
			></span
		>
		<SettingSwitch checked={app.settings!.gpu ?? false} onchange={(v) => app.save({ gpu: v })}>
			{#if app.settings!.gpu && app.gpuBackend}
				<Badge variant="secondary">{app.gpuBackend}</Badge>
			{:else}
				<span class="text-xs text-muted-foreground">Using CPU only</span>
			{/if}
		</SettingSwitch>
	</div>

	<!-- Model loading strategy -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Model Loading</span>
		<div class="flex flex-col gap-1">
			<Select.Root
				type="single"
				value={app.settings!.model_loading}
				onValueChange={(v) => {
					if (v) app.save({ model_loading: v as ModelLoading });
				}}
			>
				<Select.Trigger class="w-full">
					{loadingModes.find((m) => m.value === app.settings!.model_loading)?.label ??
						app.settings!.model_loading}
				</Select.Trigger>
				<Select.Content>
					{#each loadingModes as mode (mode.value)}
						<Select.Item value={mode.value}>{mode.label}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			{#if app.settings!.model_loading === 'per_use'}
				<span class="text-xs text-muted-foreground">Slower — loads model every recording</span>
			{/if}
		</div>
	</div>

	<!-- Language -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Language</span>
		<Select.Root
			type="single"
			value={app.settings!.language}
			onValueChange={(v) => {
				if (v) app.save({ language: v });
			}}
		>
			<Select.Trigger class="w-full">
				{languages.find((l) => l.value === app.settings!.language)?.label ?? app.settings!.language}
			</Select.Trigger>
			<Select.Content>
				{#each languages as lang (lang.value)}
					<Select.Item value={lang.value}>{lang.label}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>

	<!-- Interrupt -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Interrupt</span>
		<SettingSwitch
			checked={app.settings!.interrupt ?? false}
			label={app.settings!.interrupt
				? 'Can start a new recording while transcribing'
				: 'Must wait for transcription before recording again'}
			onchange={(v) => app.save({ interrupt: v })}
		/>
	</div>

	<!-- Min duration -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground"
			>Min Duration<span class="font-normal text-muted-foreground/60">
				— Recordings shorter than this are discarded</span
			></span
		>
		<div class="flex items-center gap-3">
			<Slider
				type="single"
				value={app.settings!.min_duration}
				min={0}
				max={2}
				step={0.1}
				class="flex-1"
				onValueChange={(v: number) => app.save({ min_duration: v })}
			/>
			<span class="w-10 text-right text-xs text-muted-foreground tabular-nums">
				{(app.settings!.min_duration ?? 0).toFixed(1)}s
			</span>
		</div>
	</div>
</div>
