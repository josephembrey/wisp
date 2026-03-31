<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { SettingSwitch } from '$lib/components/ui/setting-switch/index.js';
	import HotkeyCapture from '$lib/components/hotkey-capture.svelte';
	import { app } from '$lib/state.svelte';
	import type { OutputMode } from '$lib/tauri';
	import { overlay } from '$lib/overlay.svelte';
	import CopyIcon from '@lucide/svelte/icons/copy';
</script>

<div class="flex flex-col gap-3">
	<!-- Output mode -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Output</span>
		<div class="flex items-center gap-3">
			<ToggleGroup.Root
				type="single"
				value={app.settings!.output_mode}
				variant="outline"
				onValueChange={(v) => {
					if (v) app.save({ output_mode: v as OutputMode });
				}}
			>
				<ToggleGroup.Item value="clipboard">Clipboard</ToggleGroup.Item>
				<ToggleGroup.Item value="paste">Type</ToggleGroup.Item>
			</ToggleGroup.Root>
			<span class="text-xs text-muted-foreground">
				{app.settings!.output_mode === 'clipboard' ? 'Copies to clipboard' : 'Types at cursor'}
			</span>
		</div>
	</div>

	<!-- Hotkey -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Hotkey</span>
		<HotkeyCapture hotkey={app.settings!.hotkey} onsave={(combo) => app.save({ hotkey: combo })} />
	</div>

	<!-- Input device -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Input Device</span>
		<Select.Root
			type="single"
			value={app.settings!.input_device || ''}
			onValueChange={(v) => {
				if (v !== undefined) app.save({ input_device: v });
			}}
		>
			<Select.Trigger class="w-full truncate">
				{app.inputDevices.find((d) => d.name === app.settings!.input_device)?.label ||
					app.settings!.input_device ||
					'Default'}
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="">Default</Select.Item>
				{#each app.inputDevices as device (device.name)}
					<Select.Item value={device.name}>{device.label}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>

	<!-- Output mode hotkey -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Output Mode Hotkey</span>
		<HotkeyCapture
			hotkey={app.settings!.output_hotkey ?? ''}
			onsave={(combo) => app.save({ output_hotkey: combo })}
		/>
	</div>

	<!-- Autostart -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Autostart</span>
		<SettingSwitch
			checked={app.settings!.autostart ?? false}
			label={app.settings!.autostart ? 'Starts with system' : 'Manual start'}
			onchange={(v) => app.save({ autostart: v })}
		/>
	</div>

	<!-- Last transcription -->
	{#if app.lastTranscription}
		<div class="flex flex-col gap-1">
			<div class="flex items-center justify-between">
				<span class="text-xs font-medium text-muted-foreground">Last Transcription</span>
				<button
					class="inline-flex h-5 items-center gap-1 rounded px-1.5 text-xs text-muted-foreground hover:bg-accent hover:text-foreground"
					onclick={() => {
						navigator.clipboard.writeText(app.lastTranscription);
						overlay.notify('copied', 750);
					}}
				>
					<CopyIcon size={12} />
					Copy
				</button>
			</div>
			<Textarea
				value={app.lastTranscription}
				disabled
				class="h-16 resize-none text-xs"
				placeholder="No transcription yet"
			/>
		</div>
	{/if}
</div>
