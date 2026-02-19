<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import { Slider } from '$lib/components/ui/slider/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import SettingRow from '$lib/components/settings/setting-row.svelte';
	import ModelSection from '$lib/components/settings/model-section.svelte';
	import HotkeyCapture from '$lib/components/settings/hotkey-capture.svelte';
	import type { Settings, ModelInfo, DownloadProgress } from '$lib/tauri';

	let {
		settings,
		gpuBackend,
		models,
		downloading = null,
		progress = null,
		onsave,
		ondownload,
		ondelete
	}: {
		settings: Settings;
		gpuBackend: string;
		models: ModelInfo[];
		downloading?: string | null;
		progress?: DownloadProgress | null;
		onsave: (updates: Partial<Settings>) => void;
		ondownload: (name: string) => void;
		ondelete: (name: string) => void;
	} = $props();

	const loadingModes = [
		{ value: 'eager', label: 'Startup' },
		{ value: 'lazy', label: 'First use' },
		{ value: 'per_use', label: 'Every use' }
	];
</script>

<div class="flex flex-col gap-3">
	<ModelSection {settings} {models} {downloading} {progress} {onsave} {ondownload} {ondelete} />

	<SettingRow label="Model Loading">
		<div class="flex flex-col gap-1">
			<Select.Root
				type="single"
				value={settings.model_loading}
				onValueChange={(v) => {
					if (v) onsave({ model_loading: v as 'eager' | 'lazy' | 'per_use' });
				}}
			>
				<Select.Trigger class="w-full">
					{loadingModes.find((m) => m.value === settings.model_loading)?.label ??
						settings.model_loading}
				</Select.Trigger>
				<Select.Content>
					{#each loadingModes as mode (mode.value)}
						<Select.Item value={mode.value}>{mode.label}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			{#if settings.model_loading === 'per_use'}
				<span class="text-xs text-muted-foreground">Slower — loads model every recording</span>
			{/if}
		</div>
	</SettingRow>

	<SettingRow label="GPU">
		<div class="flex items-center gap-3">
			<Switch checked={settings.gpu} onCheckedChange={(v) => onsave({ gpu: v })} />
			{#if settings.gpu && gpuBackend}
				<Badge variant="outline">{gpuBackend}</Badge>
			{:else}
				<span class="text-xs text-muted-foreground">Using CPU only</span>
			{/if}
		</div>
	</SettingRow>

	<SettingRow label="Interrupt">
		<div class="flex items-center gap-3">
			<Switch checked={settings.interrupt} onCheckedChange={(v) => onsave({ interrupt: v })} />
			<span class="text-xs text-muted-foreground">
				{settings.interrupt ? 'Re-record during transcription' : 'Wait for transcription to finish'}
			</span>
		</div>
	</SettingRow>

	<SettingRow label="Min Duration">
		<div class="flex items-center gap-3">
			<Slider
				type="single"
				value={settings.min_duration}
				min={0}
				max={2}
				step={0.1}
				class="flex-1"
				onValueChange={(v: number) => onsave({ min_duration: v })}
			/>
			<span class="w-10 text-right text-xs text-muted-foreground tabular-nums">
				{settings.min_duration.toFixed(1)}s
			</span>
		</div>
	</SettingRow>

	<SettingRow label="Output Mode Hotkey">
		<HotkeyCapture
			hotkey={settings.output_hotkey}
			onsave={(combo) => onsave({ output_hotkey: combo })}
		/>
	</SettingRow>
</div>
