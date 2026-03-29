<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { quit, hideWindow, minimizeWindow, type OverlayStatus } from '$lib/tauri';
	import { overlay } from '$lib/overlay.svelte';
	import { toggleMode, mode } from 'mode-watcher';
	import SunIcon from '@lucide/svelte/icons/sun';
	import MoonIcon from '@lucide/svelte/icons/moon';
	import MinusIcon from '@lucide/svelte/icons/minus';
	import XIcon from '@lucide/svelte/icons/x';
	import PowerIcon from '@lucide/svelte/icons/power';

	// Status badge
	const badge: Record<OverlayStatus, { label: string; variant: string }> = {
		idle: { label: 'Idle', variant: 'default' },
		recording: { label: 'Recording', variant: 'destructive' },
		processing: { label: 'Processing', variant: 'secondary' },
		loading: { label: 'Loading', variant: 'secondary' },
		saved: { label: 'Saved', variant: 'outline' },
		copied: { label: 'Copied', variant: 'outline' },
		typed: { label: 'Typed', variant: 'outline' },
		deleted: { label: 'Deleted', variant: 'outline' },
		cancelled: { label: 'Cancelled', variant: 'outline' }
	};
	let badgeLabel = $derived(badge[overlay.current.status].label);
	let badgeVariant = $derived(badge[overlay.current.status].variant);

	// Window controls
	const btn =
		'inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground';
</script>

<!-- Titlebar: draggable, 32px height -->
<div class="flex h-8 shrink-0 items-center justify-between px-3" data-tauri-drag-region>
	<!-- App name + status badge -->
	<div class="pointer-events-none flex items-center gap-2 select-none" data-tauri-drag-region>
		<span class="text-sm font-semibold" data-tauri-drag-region>Wisp</span>
		<Badge
			variant={badgeVariant as 'default' | 'outline' | 'destructive' | 'secondary'}
			class="transition-all duration-300"
		>
			{badgeLabel}
		</Badge>
	</div>

	<!-- Window controls -->
	<div class="flex items-center gap-0.5">
		<button class={btn} onclick={() => toggleMode()}>
			{#if mode.current === 'dark'}
				<SunIcon size={12} />
			{:else}
				<MoonIcon size={12} />
			{/if}
		</button>
		<button class={btn} onclick={() => minimizeWindow()}>
			<MinusIcon size={12} />
		</button>
		<button class={btn} onclick={() => hideWindow()}>
			<XIcon size={12} />
		</button>

		<!-- Quit confirmation dialog -->
		<AlertDialog.Root>
			<AlertDialog.Trigger
				class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-destructive hover:text-white"
			>
				<PowerIcon size={12} />
			</AlertDialog.Trigger>
			<AlertDialog.Content>
				<AlertDialog.Header>
					<AlertDialog.Title>Quit Wisp?</AlertDialog.Title>
					<AlertDialog.Description>
						This will fully close Wisp. To keep it running in the background, hide to tray instead.
					</AlertDialog.Description>
				</AlertDialog.Header>
				<AlertDialog.Footer>
					<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action variant="outline" onclick={() => hideWindow()}>
						Hide to tray
					</AlertDialog.Action>
					<AlertDialog.Action onclick={() => quit()}>Quit</AlertDialog.Action>
				</AlertDialog.Footer>
			</AlertDialog.Content>
		</AlertDialog.Root>
	</div>
</div>
