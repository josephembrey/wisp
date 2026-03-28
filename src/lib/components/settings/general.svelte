<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import SettingRow from '$lib/components/settings/setting-row.svelte';
	import HotkeyCapture from '$lib/components/settings/hotkey-capture.svelte';
	import type { Settings, InputDeviceInfo } from '$lib/tauri';

	let {
		settings,
		inputDevices,
		lastTranscription,
		onsave,
		onsavedflag
	}: {
		settings: Settings;
		inputDevices: InputDeviceInfo[];
		lastTranscription: string;
		onsave: (updates: Partial<Settings>) => void;
		onsavedflag: (show: boolean, timeout?: number) => void;
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
</script>

<div class="flex flex-col gap-3">
	<SettingRow label="Output">
		<div class="flex items-center gap-3">
			<ToggleGroup.Root
				type="single"
				value={settings.output_mode}
				variant="outline"
				onValueChange={(v) => {
					if (v) onsave({ output_mode: v as 'clipboard' | 'paste' });
				}}
			>
				<ToggleGroup.Item value="clipboard">Clipboard</ToggleGroup.Item>
				<ToggleGroup.Item value="paste">Type</ToggleGroup.Item>
			</ToggleGroup.Root>
			<span class="text-xs text-muted-foreground">
				{settings.output_mode === 'clipboard' ? 'Copies to clipboard' : 'Types at cursor'}
			</span>
		</div>
	</SettingRow>

	<SettingRow label="Hotkey">
		<HotkeyCapture hotkey={settings.hotkey} onsave={(combo) => onsave({ hotkey: combo })} />
	</SettingRow>

	<SettingRow label="Input Device">
		<Select.Root
			type="single"
			value={settings.input_device || ''}
			onValueChange={(v) => {
				onsave({ input_device: v === '' ? '' : v });
			}}
		>
			<Select.Trigger class="w-full truncate">
				{inputDevices.find((d) => d.name === settings.input_device)?.label ||
					settings.input_device ||
					'Default'}
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="">Default</Select.Item>
				{#each inputDevices as device (device.name)}
					<Select.Item value={device.name}>{device.label}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</SettingRow>

	<SettingRow label="Language">
		<Select.Root
			type="single"
			value={settings.language}
			onValueChange={(v) => {
				if (v) onsave({ language: v });
			}}
		>
			<Select.Trigger class="w-full">
				{languages.find((l) => l.value === settings.language)?.label ?? settings.language}
			</Select.Trigger>
			<Select.Content>
				{#each languages as lang (lang.value)}
					<Select.Item value={lang.value}>{lang.label}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</SettingRow>

	{#if lastTranscription}
		<div class="flex flex-col gap-1">
			<div class="flex items-center justify-between">
				<span class="text-xs font-medium text-muted-foreground">Last Transcription</span>
				<button
					class="inline-flex h-5 items-center gap-1 rounded px-1.5 text-xs text-muted-foreground hover:bg-accent hover:text-foreground"
					onclick={() => {
						navigator.clipboard.writeText(lastTranscription);
						onsavedflag(true, 750);
					}}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="12"
						height="12"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><rect width="14" height="14" x="8" y="8" rx="2" /><path
							d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"
						/></svg
					>
					Copy
				</button>
			</div>
			<Textarea
				value={lastTranscription}
				disabled
				class="h-16 resize-none text-xs"
				placeholder="No transcription yet"
			/>
		</div>
	{/if}
</div>
