<script lang="ts">
	import {
		getSettings,
		updateSettings,
		getStatus,
		onStatusChanged,
		onTranscription,
		onError,
		onSettingsChanged,
		getGpuBackend,
		resetApp,
		type Settings,
		type Status
	} from '$lib/tauri';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { LogicalSize } from '@tauri-apps/api/dpi';
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Slider } from '$lib/components/ui/slider/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { toast } from 'svelte-sonner';
	import Titlebar from '$lib/components/titlebar.svelte';
	import SettingRow from '$lib/components/setting-row.svelte';
	import ModelSection from '$lib/components/model-section.svelte';
	import HotkeyCapture from '$lib/components/hotkey-capture.svelte';

	let settings: Settings | null = $state(null);
	let status: Status = $state('idle');
	let lastTranscription: string = $state('');
	let contentHeight = $state(0);
	let gpuBackend: string = $state('');

	const WINDOW_WIDTH = 400;

	// Auto-resize window to fit content
	$effect(() => {
		if (contentHeight > 0) {
			getCurrentWindow().setSize(new LogicalSize(WINDOW_WIDTH, contentHeight));
		}
	});

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

	async function save(updates: Partial<Settings>) {
		if (!settings) return;
		settings = { ...settings, ...updates };
		try {
			await updateSettings(settings);
		} catch (e) {
			toast.error(`Failed to save settings: ${e}`);
		}
	}

	$effect(() => {
		getSettings().then((s) => (settings = s));
		getStatus().then((s) => (status = s));
		getGpuBackend().then((b) => (gpuBackend = b));

		const unsubs = [
			onStatusChanged((s) => (status = s)),
			onTranscription((t) => (lastTranscription = t)),
			onError((msg) => toast.error(msg)),
			onSettingsChanged(() => {
				getSettings().then((s) => (settings = s));
			})
		];

		return () => {
			unsubs.forEach((p) => p.then((fn) => fn()));
		};
	});
</script>

<div bind:clientHeight={contentHeight}>
	<Titlebar {status} />

	{#if settings}
		<div class="flex flex-col gap-4 p-4">
			<Tabs.Root value="main">
				<Tabs.List class="w-full">
					<Tabs.Trigger value="main">Main</Tabs.Trigger>
					<Tabs.Trigger value="advanced">Advanced</Tabs.Trigger>
				</Tabs.List>

				<Tabs.Content value="main" class="flex-none">
					<div class="flex flex-col gap-4 pt-2">
						<SettingRow label="Output">
							<div class="flex items-center gap-3">
								<ToggleGroup.Root
									type="single"
									value={settings.output_mode}
									variant="outline"
									onValueChange={(v) => {
										if (v) save({ output_mode: v as 'clipboard' | 'paste' });
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
							<HotkeyCapture hotkey={settings.hotkey} onsave={(combo) => save({ hotkey: combo })} />
						</SettingRow>

						<Separator />

						<div class="flex flex-col gap-1">
							<span class="text-xs font-medium tracking-wide text-muted-foreground uppercase">
								Last Transcription
							</span>
							<Textarea
								value={lastTranscription}
								disabled
								class="h-24 resize-none text-sm"
								placeholder="No transcription yet"
							/>
						</div>
					</div>
				</Tabs.Content>

				<Tabs.Content value="advanced" class="flex-none">
					<div class="flex flex-col gap-4 pt-2">
						<ModelSection {settings} onsave={save} />

						<Separator />

						<SettingRow label="Language">
							<div class="flex items-center gap-3">
								<Select.Root
									type="single"
									value={settings.language}
									onValueChange={(v) => {
										if (v) save({ language: v });
									}}
								>
									<Select.Trigger class="w-36">
										{languages.find((l) => l.value === settings?.language)?.label ??
											settings?.language}
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
								<Switch checked={settings.gpu} onCheckedChange={(v) => save({ gpu: v })} />
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
								<Switch
									checked={settings.interrupt}
									onCheckedChange={(v) => save({ interrupt: v })}
								/>
								<span class="text-xs text-muted-foreground">
									{settings.interrupt
										? 'Re-record during transcription'
										: 'Wait for transcription to finish'}
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
									onValueChange={(v: number) => save({ min_duration: v })}
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
								onsave={(combo) => save({ output_hotkey: combo })}
							/>
						</SettingRow>

						<Separator />

						<AlertDialog.Root>
							<AlertDialog.Trigger
								class="text-xs text-muted-foreground underline hover:text-foreground"
							>
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
				</Tabs.Content>
			</Tabs.Root>
		</div>
	{/if}
</div>
