<script lang="ts">
	import {
		getSettings,
		updateSettings,
		getStatus,
		onStatusChanged,
		onTranscription,
		onError,
		onSettingsChanged,
		onOverlayFlash,
		getGpuBackend,
		getMonitors,
		getInputDevices,
		isFirstRun,
		resetApp,
		resizeWindow as resizeWindowCmd,
		hotkeyPress,
		hotkeyRelease,
		type Settings,
		type Status,
		type MonitorInfo,
		type InputDeviceInfo
	} from '$lib/tauri';
	import { tick } from 'svelte';
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
	let contentEl: HTMLDivElement | undefined = $state();
	let gpuBackend: string = $state('');
	let monitors: MonitorInfo[] = $state([]);
	let inputDevices: InputDeviceInfo[] = $state([]);
	let isDownloading: boolean = $state(false);
	let defaultTab: string = $state('main');
	let showSaved: boolean = $state(false);
	let flashMessage: string = $state('');
	let savedTimeout: ReturnType<typeof setTimeout> | undefined;
	let flashTimeout: ReturnType<typeof setTimeout> | undefined;

	let lastHeight = 0;

	// Auto-resize window to fit content
	async function resizeWindow() {
		await tick();
		if (!contentEl) return;
		const h = Math.ceil(contentEl.getBoundingClientRect().height);
		if (h > 0 && Math.abs(h - lastHeight) >= 2) {
			lastHeight = h;
			resizeWindowCmd(h);
		}
	}

	$effect(() => {
		if (!contentEl) return;

		const ro = new ResizeObserver(() => resizeWindow());
		ro.observe(contentEl);
		resizeWindow();

		return () => ro.disconnect();
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
			clearTimeout(savedTimeout);
			showSaved = true;
			savedTimeout = setTimeout(() => (showSaved = false), 750);
		} catch (e) {
			toast.error(`Failed to save settings: ${e}`);
		}
	}

	$effect(() => {
		getSettings().then((s) => (settings = s));
		getStatus().then((s) => (status = s));
		getGpuBackend().then((b) => (gpuBackend = b));
		getMonitors().then((m) => (monitors = m));
		getInputDevices().then((d) => (inputDevices = d));
		isFirstRun().then((first) => {
			if (first) defaultTab = 'readme';
		});

		const unsubs = [
			onStatusChanged((s) => (status = s)),
			onTranscription((t) => (lastTranscription = t)),
			onError((msg) => toast.error(msg)),
			onSettingsChanged(() => {
				getSettings().then((s) => (settings = s));
			}),
			onOverlayFlash((msg) => {
				clearTimeout(flashTimeout);
				flashMessage = msg;
				flashTimeout = setTimeout(() => (flashMessage = ''), 1000);
			})
		];

		return () => {
			unsubs.forEach((p) => p.then((fn) => fn()));
		};
	});

	// JS-side hotkey fallback: rdev doesn't receive key events when WebView2 is focused
	import { mapBrowserKey } from '$lib/keys';

	let pressedKeys = new Set<string>();
	let hotkeyActive = false;

	function handleKeydown(e: KeyboardEvent) {
		const key = mapBrowserKey(e.code);
		const combo = settings?.hotkey?.split('+') || [];
		// Only intercept keys that are part of the hotkey combo
		if (combo.includes(key)) {
			e.preventDefault();
		}
		pressedKeys.add(key);
		if (!hotkeyActive && combo.length > 0 && combo.every((k) => pressedKeys.has(k))) {
			hotkeyActive = true;
			hotkeyPress();
		}
	}

	function handleKeyup(e: KeyboardEvent) {
		const key = mapBrowserKey(e.code);
		if (hotkeyActive && (settings?.hotkey?.split('+') || []).includes(key)) {
			hotkeyActive = false;
			hotkeyRelease();
		}
		pressedKeys.delete(key);
	}
</script>

<svelte:window onkeydown={handleKeydown} onkeyup={handleKeyup} />

<div bind:this={contentEl}>
	<Titlebar {status} {showSaved} downloading={isDownloading} {flashMessage} />

	{#if settings}
		<div class="flex flex-col gap-4 p-4">
			<Tabs.Root value={defaultTab}>
				<Tabs.List class="w-full">
					<Tabs.Trigger value="main">Main</Tabs.Trigger>
					<Tabs.Trigger value="advanced">Advanced</Tabs.Trigger>
					<Tabs.Trigger value="overlay">Overlay</Tabs.Trigger>
					<Tabs.Trigger value="readme">About</Tabs.Trigger>
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

						<SettingRow label="Input Device">
							<Select.Root
								type="single"
								value={settings.input_device || ''}
								onValueChange={(v) => {
									save({ input_device: v === '' ? '' : v });
								}}
							>
								<Select.Trigger class="w-48 truncate">
									{inputDevices.find((d) => d.name === settings?.input_device)?.label ||
										settings?.input_device ||
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
											clearTimeout(savedTimeout);
											showSaved = true;
											savedTimeout = setTimeout(() => (showSaved = false), 750);
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
				</Tabs.Content>

				<Tabs.Content value="advanced" class="flex-none">
					<div class="flex flex-col gap-4 pt-2">
						<ModelSection {settings} onsave={save} ondownloadchange={(v) => (isDownloading = v)} />

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

				<Tabs.Content value="overlay" class="flex-none">
					<div class="flex flex-col gap-4 pt-2">
						<SettingRow label="Enabled">
							<div class="flex items-center gap-3">
								<Switch
									checked={settings.overlay_enabled}
									onCheckedChange={(v) => save({ overlay_enabled: v })}
								/>
								<span class="text-xs text-muted-foreground">
									{settings.overlay_enabled ? 'Shows status pill' : 'Overlay hidden'}
								</span>
							</div>
						</SettingRow>

						<Separator />

						<SettingRow label="Visibility">
							<div class="flex items-center gap-3">
								<Switch
									checked={settings.overlay_always_show}
									onCheckedChange={(v) => save({ overlay_always_show: v })}
								/>
								<span class="text-xs text-muted-foreground">
									{settings.overlay_always_show ? 'Always visible' : 'Only when active'}
								</span>
							</div>
						</SettingRow>

						<Separator />

						<SettingRow label="Position">
							<Select.Root
								type="single"
								value={settings.overlay_position}
								onValueChange={(v) => {
									if (v) save({ overlay_position: v });
								}}
							>
								<Select.Trigger class="w-36">
									{(
										{
											'top-left': 'Top Left',
											'top-center': 'Top Center',
											'top-right': 'Top Right',
											'bottom-left': 'Bottom Left',
											'bottom-center': 'Bottom Center',
											'bottom-right': 'Bottom Right'
										} as Record<string, string>
									)[settings.overlay_position] ?? settings.overlay_position}
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

						<Separator />

						<SettingRow label="Size">
							<ToggleGroup.Root
								type="single"
								value={settings.overlay_size}
								variant="outline"
								onValueChange={(v) => {
									if (v) save({ overlay_size: v });
								}}
							>
								<ToggleGroup.Item value="small">Small</ToggleGroup.Item>
								<ToggleGroup.Item value="medium">Medium</ToggleGroup.Item>
								<ToggleGroup.Item value="large">Large</ToggleGroup.Item>
							</ToggleGroup.Root>
						</SettingRow>

						<Separator />

						<SettingRow label="Monitor">
							<Select.Root
								type="single"
								value={String(settings.overlay_monitor)}
								onValueChange={(v) => {
									if (v !== undefined) save({ overlay_monitor: Number(v) });
								}}
							>
								<Select.Trigger class="w-48 truncate">
									{(() => {
										const m = monitors.find((m) => m.index === settings?.overlay_monitor);
										if (!m) return `Monitor ${settings.overlay_monitor}`;
										return `${m.name || `Monitor ${m.index}`}${m.primary ? ' (Primary)' : ''} - ${m.width}x${m.height}`;
									})()}
								</Select.Trigger>
								<Select.Content>
									{#each monitors as monitor (monitor.index)}
										<Select.Item value={String(monitor.index)}>
											{monitor.name || `Monitor ${monitor.index}`}{monitor.primary
												? ' (Primary)'
												: ''} - {monitor.width}x{monitor.height}
										</Select.Item>
									{/each}
								</Select.Content>
							</Select.Root>
						</SettingRow>
					</div>
				</Tabs.Content>

				<Tabs.Content value="readme" class="flex-none">
					<div class="flex flex-col gap-4 pt-2">
						<p class="text-sm leading-relaxed text-muted-foreground">
							Wisp is a push-to-talk dictation app. Hold a hotkey to record, release to transcribe
							locally with Whisper, and the text goes to your clipboard or cursor.
						</p>

						<Separator />

						<div class="flex flex-col gap-2">
							<span class="text-xs font-semibold tracking-wide text-foreground uppercase"
								>Quick Start</span
							>
							<ol class="space-y-2 text-xs text-muted-foreground">
								<li class="flex gap-2">
									<span class="font-semibold text-foreground">1.</span>
									<span>Download a model in <strong>Advanced</strong> (base is a good start)</span>
								</li>
								<li class="flex gap-2">
									<span class="font-semibold text-foreground">2.</span>
									<span
										>Hold <strong>{settings.hotkey.replace(/\+/g, ' + ')}</strong> to record</span
									>
								</li>
								<li class="flex gap-2">
									<span class="font-semibold text-foreground">3.</span>
									<span>Release to transcribe</span>
								</li>
							</ol>
						</div>

						<Separator />

						<div class="flex flex-col gap-2">
							<span class="text-xs font-semibold tracking-wide text-foreground uppercase">Tips</span
							>
							<ul class="space-y-1.5 text-xs text-muted-foreground">
								<li class="flex items-start gap-2">
									<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
									<span>Minimize this window — Wisp keeps running in the system tray</span>
								</li>
								<li class="flex items-start gap-2">
									<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
									<span
										><strong>Clipboard</strong> copies text, <strong>Type</strong> pastes at your cursor</span
									>
								</li>
								<li class="flex items-start gap-2">
									<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
									<span>Larger models are slower but more accurate</span>
								</li>
								<li class="flex items-start gap-2">
									<span class="mt-1.5 h-1 w-1 shrink-0 rounded-full bg-muted-foreground"></span>
									<span>Enable GPU in Advanced for faster transcription</span>
								</li>
							</ul>
						</div>
					</div>
				</Tabs.Content>
			</Tabs.Root>
		</div>
	{/if}
</div>
