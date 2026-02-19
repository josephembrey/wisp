<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { quit, hideWindow, minimizeWindow, type Status } from '$lib/tauri';

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
	<div class="flex items-center gap-0.5">
		<button
			class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
			onclick={() => minimizeWindow()}
			aria-label="Minimize"
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
				stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12" /></svg
			>
		</button>
		<button
			class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
			onclick={() => hideWindow()}
			aria-label="Close to tray"
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
				><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
			>
		</button>
		<AlertDialog.Root>
			<AlertDialog.Trigger
				class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-destructive hover:text-white"
				aria-label="Quit"
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
					><path d="M18.36 6.64a9 9 0 1 1-12.73 0" /><line x1="12" y1="2" x2="12" y2="12" /></svg
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
