<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import { quit, hideWindow, minimizeWindow, type OverlayIcon } from '$lib/tauri';
	import { app } from '$lib/state.svelte';
	import { overlay } from '$lib/overlay.svelte';
	import { toggleMode, mode } from 'mode-watcher';
	import SunIcon from '@lucide/svelte/icons/sun';
	import MoonIcon from '@lucide/svelte/icons/moon';
	import MinusIcon from '@lucide/svelte/icons/minus';
	import XIcon from '@lucide/svelte/icons/x';
	import PowerIcon from '@lucide/svelte/icons/power';

	const iconVariant: Record<OverlayIcon, string> = {
		dot: 'default',
		pulse: 'destructive',
		spinner: 'secondary',
		check: 'outline',
		x: 'outline'
	};

	let badgeLabel = $derived(overlay.current.label);
	let badgeVariant = $derived(iconVariant[overlay.current.icon]);
</script>

<div class="flex h-8 shrink-0 items-center justify-between px-3" data-tauri-drag-region>
	<div class="pointer-events-none flex items-center gap-2 select-none" data-tauri-drag-region>
		<span class="text-sm font-semibold" data-tauri-drag-region>Wisp</span>
		<Badge
			variant={badgeVariant as 'default' | 'outline' | 'destructive' | 'secondary'}
			class="transition-all duration-300"
		>
			{badgeLabel}
		</Badge>
	</div>
	<Tooltip.Provider delayDuration={400}>
		<div class="flex items-center gap-0.5">
			<Tooltip.Root>
				<Tooltip.Trigger>
					{#snippet child({ props })}
						<div {...props} class="inline-flex items-center">
							<div
								class="cursor-pointer"
								role="switch"
								tabindex="0"
								aria-checked={app.settings?.autostart ?? false}
								onclick={() => app.save({ autostart: !(app.settings?.autostart ?? false) })}
								onkeydown={(e) => {
									if (e.key === 'Enter' || e.key === ' ') {
										e.preventDefault();
										app.save({ autostart: !(app.settings?.autostart ?? false) });
									}
								}}
							>
								<Switch
									checked={app.settings?.autostart ?? false}
									class="pointer-events-none scale-75"
								/>
							</div>
						</div>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>Toggle Autostart</p></Tooltip.Content>
			</Tooltip.Root>

			<Tooltip.Root>
				<Tooltip.Trigger>
					{#snippet child({ props })}
						<button
							{...props}
							class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
							onclick={() => toggleMode()}
						>
							{#if mode.current === 'dark'}
								<SunIcon size={12} />
							{:else}
								<MoonIcon size={12} />
							{/if}
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>Toggle theme</p></Tooltip.Content>
			</Tooltip.Root>

			<Tooltip.Root>
				<Tooltip.Trigger>
					{#snippet child({ props })}
						<button
							{...props}
							class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
							onclick={() => minimizeWindow()}
						>
							<MinusIcon size={12} />
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>Minimize</p></Tooltip.Content>
			</Tooltip.Root>

			<Tooltip.Root>
				<Tooltip.Trigger>
					{#snippet child({ props })}
						<button
							{...props}
							class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
							onclick={() => hideWindow()}
						>
							<XIcon size={12} />
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>Hide to tray</p></Tooltip.Content>
			</Tooltip.Root>

			<AlertDialog.Root>
				<Tooltip.Root>
					<Tooltip.Trigger>
						{#snippet child({ props })}
							<AlertDialog.Trigger
								{...props}
								class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-destructive hover:text-white"
							>
								<PowerIcon size={12} />
							</AlertDialog.Trigger>
						{/snippet}
					</Tooltip.Trigger>
					<Tooltip.Content><p>Quit</p></Tooltip.Content>
				</Tooltip.Root>
				<AlertDialog.Content>
					<AlertDialog.Header>
						<AlertDialog.Title>Quit Wisp?</AlertDialog.Title>
						<AlertDialog.Description>
							This will stop the hotkey listener and close the app.
						</AlertDialog.Description>
					</AlertDialog.Header>
					<AlertDialog.Footer>
						<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
						<AlertDialog.Action onclick={() => quit()}>Quit</AlertDialog.Action>
					</AlertDialog.Footer>
				</AlertDialog.Content>
			</AlertDialog.Root>
		</div>
	</Tooltip.Provider>
</div>
