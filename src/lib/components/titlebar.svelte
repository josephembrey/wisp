<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { toggleMode, mode } from 'mode-watcher';
	import { quit, hideWindow, type Status } from '$lib/tauri';

	let {
		status,
		showSaved = false,
		downloading = false,
		flashMessage = ''
	}: {
		status: Status;
		showSaved?: boolean;
		downloading?: boolean;
		flashMessage?: string;
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
	class="flex h-9 shrink-0 items-center justify-between border-b border-border bg-card px-3"
	data-tauri-drag-region
>
	<div class="pointer-events-none flex items-center gap-2 select-none" data-tauri-drag-region>
		<span class="text-sm font-medium" data-tauri-drag-region>Wisp</span>
		<Badge
			variant={badgeVariant as 'default' | 'outline' | 'destructive' | 'secondary'}
			class="transition-all duration-300"
		>
			{badgeLabel}
		</Badge>
	</div>
	<div class="flex items-center gap-1">
		<button
			class="inline-flex h-6 w-6 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
			onclick={() => toggleMode()}
			aria-label="Toggle theme"
		>
			{#if mode.current === 'dark'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					><circle cx="12" cy="12" r="5" /><line x1="12" y1="1" x2="12" y2="3" /><line
						x1="12"
						y1="21"
						x2="12"
						y2="23"
					/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64" /><line
						x1="18.36"
						y1="18.36"
						x2="19.78"
						y2="19.78"
					/><line x1="1" y1="12" x2="3" y2="12" /><line x1="21" y1="12" x2="23" y2="12" /><line
						x1="4.22"
						y1="19.78"
						x2="5.64"
						y2="18.36"
					/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" /></svg
				>
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" /></svg
				>
			{/if}
		</button>
		<button
			class="inline-flex h-6 w-6 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
			onclick={() => hideWindow()}
			aria-label="Hide to tray"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12" /></svg
			>
		</button>
		<AlertDialog.Root>
			<AlertDialog.Trigger
				class="inline-flex h-6 w-6 items-center justify-center rounded-sm text-muted-foreground hover:bg-destructive hover:text-white"
				aria-label="Quit"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
				>
			</AlertDialog.Trigger>
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
</div>
