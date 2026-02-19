<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import SettingRow from '$lib/components/setting-row.svelte';
	import HotkeyCapture from '$lib/components/hotkey-capture.svelte';
	import type { Settings, InputDeviceInfo } from '$lib/tauri';

	let {
		settings,
		inputDevices,
		lastTranscription,
		showSaved,
		onsave,
		onsavedflag
	}: {
		settings: Settings;
		inputDevices: InputDeviceInfo[];
		lastTranscription: string;
		showSaved: boolean;
		onsave: (updates: Partial<Settings>) => void;
		onsavedflag: (show: boolean, timeout?: number) => void;
	} = $props();
</script>

<div class="flex flex-col gap-4">
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

	<Separator />

	<SettingRow label="Hotkey">
		<HotkeyCapture hotkey={settings.hotkey} onsave={(combo) => onsave({ hotkey: combo })} />
	</SettingRow>

	<Separator />

	<SettingRow label="Input Device">
		<Select.Root
			type="single"
			value={settings.input_device || ''}
			onValueChange={(v) => {
				onsave({ input_device: v === '' ? '' : v });
			}}
		>
			<Select.Trigger class="w-48 truncate">
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

	<Separator />

	<div class="flex flex-col gap-1">
		<div class="flex items-center justify-between">
			<span class="text-xs font-medium tracking-wide text-muted-foreground uppercase">
				Last Transcription
			</span>
			{#if lastTranscription}
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
			{/if}
		</div>
		<Textarea
			value={lastTranscription}
			disabled
			class="h-24 resize-none text-sm"
			placeholder="No transcription yet"
		/>
	</div>
</div>
