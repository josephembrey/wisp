<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { SettingSwitch } from '$lib/components/ui/setting-switch/index.js';
	import {
		getHistory,
		clearHistory,
		deleteHistoryEntry,
		onHistoryChanged,
		type HistoryEntry
	} from '$lib/tauri';
	import { onMount } from 'svelte';
	import { log } from '$lib/log';
	import { app } from '$lib/state.svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import CopyIcon from '@lucide/svelte/icons/copy';
	import XIcon from '@lucide/svelte/icons/x';

	let entries: HistoryEntry[] = $state([]);
	let expandedId: number | null = $state(null);
	let copiedId: number | null = $state(null);
	let copiedTimeout: ReturnType<typeof setTimeout> | undefined;

	async function refresh() {
		try {
			entries = await getHistory();
		} catch (e) {
			log.error(`[history] failed to load: ${e}`);
		}
	}

	async function handleDelete(id: number) {
		try {
			await deleteHistoryEntry(id);
			entries = entries.filter((e) => e.id !== id);
		} catch (e) {
			log.error(`[history] delete failed: ${e}`);
		}
	}

	async function handleClearAll() {
		try {
			await clearHistory();
			entries = [];
		} catch (e) {
			log.error(`[history] clear failed: ${e}`);
		}
	}

	function copyText(entry: HistoryEntry) {
		navigator.clipboard.writeText(entry.text);
		clearTimeout(copiedTimeout);
		copiedId = entry.id;
		copiedTimeout = setTimeout(() => (copiedId = null), 750);
	}

	function timeAgo(timestamp: number): string {
		const now = Math.floor(Date.now() / 1000);
		const diff = now - timestamp;
		if (diff < 60) return 'just now';
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		if (diff < 604800) return `${Math.floor(diff / 86400)}d ago`;
		return new Date(timestamp * 1000).toLocaleDateString();
	}

	onMount(() => {
		refresh();
		const unsub = onHistoryChanged(() => refresh());
		return () => {
			unsub.then((fn) => fn());
		};
	});
</script>

<div class="flex flex-col gap-2">
	<div class="flex items-center justify-between gap-2">
		<SettingSwitch
			checked={app.settings!.history_enabled ?? true}
			label="Save history"
			onchange={(v) => app.save({ history_enabled: v })}
		/>
		<div
			class="flex items-center gap-1.5 transition-opacity {app.settings!.history_enabled
				? ''
				: 'pointer-events-none opacity-40'}"
		>
			<span class="text-xs text-muted-foreground">Keep</span>
			<input
				type="number"
				min="10"
				max="10000"
				step="10"
				disabled={!app.settings!.history_enabled}
				value={app.settings!.history_retention ?? 100}
				onchange={(e) => {
					const v = parseInt(e.currentTarget.value);
					if (!isNaN(v) && v >= 10) app.save({ history_retention: v });
				}}
				class="h-6 w-16 rounded-md border border-input bg-background px-1.5 text-xs"
			/>
			<span class="text-xs text-muted-foreground">entries</span>
		</div>
	</div>

	{#if entries.length === 0}
		<p class="py-6 text-center text-xs text-muted-foreground">No transcription history yet.</p>
	{:else}
		<div class="flex max-h-64 flex-col gap-1 overflow-y-auto pr-1">
			{#each entries as entry (entry.id)}
				<div class="group rounded-md border border-border bg-background p-2">
					<div class="flex items-start justify-between gap-2">
						<button
							class="min-w-0 flex-1 text-left"
							onclick={() => (expandedId = expandedId === entry.id ? null : entry.id)}
						>
							<div class="flex items-center gap-1.5">
								<span
									class="shrink-0 rounded px-1 py-0.5 text-[10px] leading-none font-medium uppercase {entry.source ===
									'mic'
										? 'bg-primary/10 text-primary'
										: 'bg-secondary text-secondary-foreground'}"
								>
									{entry.source}
								</span>
								<span class="text-[10px] text-muted-foreground">{timeAgo(entry.timestamp)}</span>
							</div>
							<p class="mt-1 text-xs text-foreground" class:line-clamp-1={expandedId !== entry.id}>
								{entry.text}
							</p>
						</button>
						<div
							class="flex shrink-0 items-center gap-0.5 opacity-0 transition-opacity group-hover:opacity-100"
						>
							<button
								class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-accent hover:text-foreground"
								onclick={() => copyText(entry)}
								title="Copy"
							>
								{#if copiedId === entry.id}
									<CheckIcon size={12} />
								{:else}
									<CopyIcon size={12} />
								{/if}
							</button>
							<button
								class="inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground hover:bg-destructive hover:text-white"
								onclick={() => handleDelete(entry.id)}
								title="Delete"
							>
								<XIcon size={12} />
							</button>
						</div>
					</div>
				</div>
			{/each}
		</div>

		<AlertDialog.Root>
			<AlertDialog.Trigger
				class="self-start text-xs text-muted-foreground underline hover:text-foreground"
			>
				Clear all history
			</AlertDialog.Trigger>
			<AlertDialog.Content>
				<AlertDialog.Header>
					<AlertDialog.Title>Clear History?</AlertDialog.Title>
					<AlertDialog.Description>
						This will permanently delete all {entries.length} transcription entries.
					</AlertDialog.Description>
				</AlertDialog.Header>
				<AlertDialog.Footer>
					<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action onclick={handleClearAll}>Clear</AlertDialog.Action>
				</AlertDialog.Footer>
			</AlertDialog.Content>
		</AlertDialog.Root>
	{/if}
</div>
