<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import { Slider } from '$lib/components/ui/slider/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import SettingRow from '$lib/components/setting-row.svelte';
	import ModelSection from '$lib/components/model-section.svelte';
	import HotkeyCapture from '$lib/components/hotkey-capture.svelte';
	import { resetApp, type Settings } from '$lib/tauri';

	let {
		settings,
		gpuBackend,
		onsave,
		ondownloadchange
	}: {
		settings: Settings;
		gpuBackend: string;
		onsave: (updates: Partial<Settings>) => void;
		ondownloadchange: (downloading: boolean) => void;
	} = $props();

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

	const loadingModes = [
		{ value: 'eager', label: 'Startup' },
		{ value: 'lazy', label: 'First use' },
		{ value: 'per_use', label: 'Every use' }
	];
</script>

<div class="flex flex-col gap-4">
	<ModelSection {settings} {onsave} ondownloadchange={(v) => ondownloadchange(v)} />

	<Separator />

	<SettingRow label="Model Loading">
		<div class="flex flex-col gap-1">
			<Select.Root
				type="single"
				value={settings.model_loading}
				onValueChange={(v) => {
					if (v) onsave({ model_loading: v as 'eager' | 'lazy' | 'per_use' });
				}}
			>
				<Select.Trigger class="w-36">
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

	<Separator />

	<SettingRow label="Language">
		<div class="flex items-center gap-3">
			<Select.Root
				type="single"
				value={settings.language}
				onValueChange={(v) => {
					if (v) onsave({ language: v });
				}}
			>
				<Select.Trigger class="w-36">
					{languages.find((l) => l.value === settings.language)?.label ?? settings.language}
				</Select.Trigger>
				<Select.Content>
					{#each languages as lang (lang.value)}
						<Select.Item value={lang.value}>{lang.label}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
	</SettingRow>

	<Separator />

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

	<Separator />

	<SettingRow label="Interrupt">
		<div class="flex items-center gap-3">
			<Switch checked={settings.interrupt} onCheckedChange={(v) => onsave({ interrupt: v })} />
			<span class="text-xs text-muted-foreground">
				{settings.interrupt ? 'Re-record during transcription' : 'Wait for transcription to finish'}
			</span>
		</div>
	</SettingRow>

	<Separator />

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

	<Separator />

	<SettingRow label="Output Mode Hotkey">
		<HotkeyCapture
			hotkey={settings.output_hotkey}
			onsave={(combo) => onsave({ output_hotkey: combo })}
		/>
	</SettingRow>

	<Separator />

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
