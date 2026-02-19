<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	import { quit, hideWindow, minimizeWindow, type Status } from '$lib/tauri';
	import { toggleMode, mode } from 'mode-watcher';

	let {
		status,
		showSaved = false,
		downloading = false,
		flashMessage = '',
		autostart = false,
		onautostart
	}: {
		status: Status;
		showSaved?: boolean;
		downloading?: boolean;
		flashMessage?: string;
		autostart?: boolean;
		onautostart?: (enabled: boolean) => void;
	} = $props();

	const statusColor: Record<Status, string> = {
		idle: 'default',
		loading: 'outline',
		recording: 'destructive',
		processing: 'secondary'
	};

	const statusLabel: Record<Status, string> = {
		idle: 'Idle',
		loading: 'Loading Model',
		recording: 'Recording',
		processing: 'Processing'
	};

	let badgeLabel = $derived(
		flashMessage
			? flashMessage
			: showSaved
				? 'Saved'
				: downloading && status === 'idle'
					? 'Downloading'
					: statusLabel[status]
	);
	let badgeVariant = $derived(
		flashMessage || showSaved
			? 'outline'
			: downloading && status === 'idle'
				? 'outline'
				: statusColor[status]
	);
</script>

<div
	class="flex h-8 shrink-0 items-center justify-between px-3"
	data-tauri-drag-region
>
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
				<Tooltip.Trigger asChild>
					{#snippet child({ props })}
						<button
							{...props}
							class="inline-flex h-5 w-5 items-center justify-center rounded-sm transition-colors {autostart ? 'text-foreground' : 'text-muted-foreground/40'} hover:bg-accent hover:text-foreground"
							onclick={() => onautostart?.(!autostart)}
						>
							<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m13 2-2 2.5h3L12 7" /><path d="M10 14v-3" /><path d="M14 14v-3" /><path d="M11 19c-1.7 0-3-1.3-3-3v-2h8v2c0 1.7-1.3 3-3 3Z" /><path d="M12 22v-3" /></svg>
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>{autostart ? 'Autostart on' : 'Autostart off'}</p></Tooltip.Content>
			</Tooltip.Root>

			<Tooltip.Root>
				<Tooltip.Trigger asChild>
					{#snippet child({ props })}
						<button
							{...props}
							class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
							onclick={() => toggleMode()}
						>
							{#if mode.current === 'dark'}
								<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4" /><path d="M12 2v2" /><path d="M12 20v2" /><path d="m4.93 4.93 1.41 1.41" /><path d="m17.66 17.66 1.41 1.41" /><path d="M2 12h2" /><path d="M20 12h2" /><path d="m6.34 17.66-1.41 1.41" /><path d="m19.07 4.93-1.41 1.41" /></svg>
							{:else}
								<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" /></svg>
							{/if}
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>Toggle theme</p></Tooltip.Content>
			</Tooltip.Root>

			<Tooltip.Root>
				<Tooltip.Trigger asChild>
					{#snippet child({ props })}
						<button
							{...props}
							class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
							onclick={() => minimizeWindow()}
						>
							<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12" /></svg>
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>Minimize</p></Tooltip.Content>
			</Tooltip.Root>

			<Tooltip.Root>
				<Tooltip.Trigger asChild>
					{#snippet child({ props })}
						<button
							{...props}
							class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
							onclick={() => hideWindow()}
						>
							<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg>
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content><p>Hide to tray</p></Tooltip.Content>
			</Tooltip.Root>

			<AlertDialog.Root>
				<Tooltip.Root>
					<Tooltip.Trigger asChild>
						{#snippet child({ props })}
							<AlertDialog.Trigger
								{...props}
								class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-destructive hover:text-white"
							>
								<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18.36 6.64a9 9 0 1 1-12.73 0" /><line x1="12" y1="2" x2="12" y2="12" /></svg>
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
