<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import SettingRow from '$lib/components/settings/setting-row.svelte';
	import type { Settings, MonitorInfo } from '$lib/tauri';

	let {
		settings,
		monitors,
		onsave
	}: {
		settings: Settings;
		monitors: MonitorInfo[];
		onsave: (updates: Partial<Settings>) => void;
	} = $props();
</script>

<div class="flex flex-col gap-3">
	<SettingRow label="Enabled">
		<div
			class="flex cursor-pointer items-center gap-3"
			role="switch"
			tabindex="0"
			aria-checked={settings.overlay_enabled}
			onclick={() => onsave({ overlay_enabled: !settings.overlay_enabled })}
			onkeydown={(e) => {
				if (e.key === 'Enter' || e.key === ' ') {
					e.preventDefault();
					onsave({ overlay_enabled: !settings.overlay_enabled });
				}
			}}
		>
			<Switch checked={settings.overlay_enabled} class="pointer-events-none" />
			<span class="text-xs text-muted-foreground">
				{settings.overlay_enabled ? 'Shows status pill' : 'Overlay hidden'}
			</span>
		</div>
	</SettingRow>

	<SettingRow label="Visibility">
		<div
			class="flex cursor-pointer items-center gap-3"
			role="switch"
			tabindex="0"
			aria-checked={settings.overlay_always_show}
			onclick={() => onsave({ overlay_always_show: !settings.overlay_always_show })}
			onkeydown={(e) => {
				if (e.key === 'Enter' || e.key === ' ') {
					e.preventDefault();
					onsave({ overlay_always_show: !settings.overlay_always_show });
				}
			}}
		>
			<Switch checked={settings.overlay_always_show} class="pointer-events-none" />
			<span class="text-xs text-muted-foreground">
				{settings.overlay_always_show ? 'Always visible' : 'Only when active'}
			</span>
		</div>
	</SettingRow>

	<SettingRow label="Position">
		<Select.Root
			type="single"
			value={settings.overlay_position}
			onValueChange={(v) => {
				if (v) onsave({ overlay_position: v });
			}}
		>
			<Select.Trigger class="w-full">
				{(
					{
						'top-left': 'Top Left',
						'top-center': 'Top Center',
						'top-right': 'Top Right',
						'bottom-left': 'Bottom Left',
						'bottom-center': 'Bottom Center',
						'bottom-right': 'Bottom Right'
					} as Record<string, string>
				)[settings.overlay_position ?? ''] ?? settings.overlay_position}
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
	</SettingRow>

	<SettingRow label="Size">
		<ToggleGroup.Root
			type="single"
			value={settings.overlay_size}
			variant="outline"
			onValueChange={(v) => {
				if (v) onsave({ overlay_size: v });
			}}
		>
			<ToggleGroup.Item value="small">Small</ToggleGroup.Item>
			<ToggleGroup.Item value="medium">Medium</ToggleGroup.Item>
			<ToggleGroup.Item value="large">Large</ToggleGroup.Item>
		</ToggleGroup.Root>
	</SettingRow>

	<SettingRow label="Monitor">
		<Select.Root
			type="single"
			value={String(settings.overlay_monitor)}
			onValueChange={(v) => {
				if (v !== undefined) onsave({ overlay_monitor: Number(v) });
			}}
		>
			<Select.Trigger class="w-full truncate">
				{(() => {
					const m = monitors.find((m) => m.index === settings.overlay_monitor);
					if (!m) return `Monitor ${settings.overlay_monitor}`;
					return `${m.name || `Monitor ${m.index}`}${m.primary ? ' (Primary)' : ''} - ${m.width}x${m.height}`;
				})()}
			</Select.Trigger>
			<Select.Content>
				{#each monitors as monitor (monitor.index)}
					<Select.Item value={String(monitor.index)}>
						{monitor.name || `Monitor ${monitor.index}`}{monitor.primary ? ' (Primary)' : ''} - {monitor.width}x{monitor.height}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</SettingRow>
</div>
