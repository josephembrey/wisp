<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { SettingSwitch } from '$lib/components/ui/setting-switch/index.js';
	import { app } from '$lib/state.svelte';
	import type { MonitorInfo } from '$lib/tauri';

	const positions: Record<string, string> = {
		'top-left': 'Top Left',
		'top-center': 'Top Center',
		'top-right': 'Top Right',
		'bottom-left': 'Bottom Left',
		'bottom-center': 'Bottom Center',
		'bottom-right': 'Bottom Right'
	};

	function monitorLabel(m: MonitorInfo) {
		return `${m.name || `Monitor ${m.index}`}${m.primary ? ' (Primary)' : ''} — ${m.width}×${m.height}`;
	}

	const selectedMonitor = $derived(
		app.monitors.find((m) => m.index === app.settings!.overlay_monitor)
	);
</script>

<div class="flex flex-col gap-3">
	<!-- Enabled -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Enabled</span>
		<SettingSwitch
			checked={app.settings!.overlay_enabled ?? true}
			label={app.settings!.overlay_enabled ? 'Shows status pill' : 'Overlay hidden'}
			onchange={(v) => app.save({ overlay_enabled: v })}
		/>
	</div>

	<!-- Visibility -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Visibility</span>
		<SettingSwitch
			checked={app.settings!.overlay_always_show ?? false}
			label={app.settings!.overlay_always_show ? 'Always visible' : 'Only when active'}
			onchange={(v) => app.save({ overlay_always_show: v })}
		/>
	</div>

	<!-- Position -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Position</span>
		<Select.Root
			type="single"
			value={app.settings!.overlay_position}
			onValueChange={(v) => {
				if (v) app.save({ overlay_position: v });
			}}
		>
			<Select.Trigger class="w-full">
				{positions[app.settings!.overlay_position ?? ''] ?? app.settings!.overlay_position}
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="top-left">Top Left</Select.Item>
				<Select.Item value="top-center">Top Center</Select.Item>
				<Select.Item value="top-right">Top Right</Select.Item>
				<Select.Item value="bottom-left">Bottom Left</Select.Item>
				<Select.Item value="bottom-center">Bottom Center</Select.Item>
				<Select.Item value="bottom-right">Bottom Right</Select.Item>
			</Select.Content>
		</Select.Root>
	</div>

	<!-- Size -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Size</span>
		<ToggleGroup.Root
			type="single"
			value={app.settings!.overlay_size}
			variant="outline"
			onValueChange={(v) => {
				if (v) app.save({ overlay_size: v });
			}}
		>
			<ToggleGroup.Item value="small">Small</ToggleGroup.Item>
			<ToggleGroup.Item value="medium">Medium</ToggleGroup.Item>
			<ToggleGroup.Item value="large">Large</ToggleGroup.Item>
		</ToggleGroup.Root>
	</div>

	<!-- Monitor -->
	<div class="flex flex-col gap-1.5">
		<span class="text-xs font-medium text-muted-foreground">Monitor</span>
		<Select.Root
			type="single"
			value={String(app.settings!.overlay_monitor)}
			onValueChange={(v) => {
				if (v !== undefined) app.save({ overlay_monitor: Number(v) });
			}}
		>
			<Select.Trigger class="w-full truncate">
				{selectedMonitor
					? monitorLabel(selectedMonitor)
					: `Monitor ${app.settings!.overlay_monitor}`}
			</Select.Trigger>
			<Select.Content>
				{#each app.monitors as monitor (monitor.index)}
					<Select.Item value={String(monitor.index)}>
						{monitorLabel(monitor)}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>
</div>
