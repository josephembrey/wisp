<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import * as Pagination from '$lib/components/ui/pagination/index.js';
	import { SettingSwitch } from '$lib/components/ui/setting-switch/index.js';
	import {
		getHistory,
		clearHistory,
		deleteHistoryEntry,
		onHistoryChanged,
		type HistoryEntry
	} from '$lib/tauri';
	import { onMount } from 'svelte';
	import { error as logError } from '@tauri-apps/plugin-log';
	import { app } from '$lib/state.svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import CopyIcon from '@lucide/svelte/icons/copy';
	import XIcon from '@lucide/svelte/icons/x';

	let entries: HistoryEntry[] = $state([]);
	let expandedId: number | null = $state(null);
	let copiedId: number | null = $state(null);
	let copiedTimeout: ReturnType<typeof setTimeout> | undefined;
	let page = $state(1);

	// Pagination
	const PER_PAGE = 5;
	let totalPages = $derived(Math.ceil(entries.length / PER_PAGE));
	let pageEntries = $derived(entries.slice((page - 1) * PER_PAGE, page * PER_PAGE));

	// Source badge styles
	const sourceBadge: Record<string, string> = {
		mic: 'bg-primary/10 text-primary',
		file: 'bg-secondary text-secondary-foreground'
	};

	// Actions
	async function refresh() {
		try {
			entries = await getHistory();
			if (page > totalPages && totalPages > 0) page = totalPages;
		} catch (e) {
			logError(`[history] failed to load: ${e}`);
		}
	}

	async function handleDelete(id: number) {
		try {
			await deleteHistoryEntry(id);
			entries = entries.filter((e) => e.id !== id);
			if (page > totalPages && totalPages > 0) page = totalPages;
		} catch (e) {
			logError(`[history] delete failed: ${e}`);
		}
	}

	async function handleClearAll() {
		try {
			await clearHistory();
			entries = [];
			page = 1;
		} catch (e) {
			logError(`[history] clear failed: ${e}`);
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

	// Lifecycle
	onMount(() => {
		refresh();
		const unsub = onHistoryChanged(() => refresh());
		return () => {
			unsub.then((fn) => fn());
		};
	});

	// Shared styles
	const actionBtn =
		'inline-flex h-5 w-5 items-center justify-center rounded-sm text-muted-foreground';
</script>

<div class="flex flex-col gap-2">
	<!-- Empty state -->
	{#if entries.length === 0}
		<div class="flex flex-col items-center gap-1 py-8 text-center">
			<p class="text-xs text-muted-foreground">No transcription history yet.</p>
			<p class="text-[10px] text-muted-foreground/60">
				Hold your hotkey to record, history will appear here.
			</p>
		</div>

		<!-- Entry list -->
	{:else}
		<!-- Pagination -->
		{#if totalPages > 1}
			<Pagination.Root count={entries.length} perPage={PER_PAGE} bind:page>
				{#snippet children({ pages, currentPage })}
					<Pagination.Content class="w-full justify-between">
						<Pagination.Item>
							<Pagination.PrevButton />
						</Pagination.Item>
						{#each pages as pg (pg.key)}
							{#if pg.type === 'ellipsis'}
								<Pagination.Item>
									<Pagination.Ellipsis />
								</Pagination.Item>
							{:else}
								<Pagination.Item>
									<Pagination.Link page={pg} isActive={currentPage === pg.value}>
										{pg.value}
									</Pagination.Link>
								</Pagination.Item>
							{/if}
						{/each}
						<Pagination.Item>
							<Pagination.NextButton />
						</Pagination.Item>
					</Pagination.Content>
				{/snippet}
			</Pagination.Root>
		{/if}

		<!-- Entries -->
		<div class="flex flex-col divide-y divide-border">
			{#each pageEntries as entry (entry.id)}
				<div class="group py-2">
					<div class="flex items-start justify-between gap-2">
						<!-- Entry content (click to expand) -->
						<button
							class="min-w-0 flex-1 text-left"
							onclick={() => (expandedId = expandedId === entry.id ? null : entry.id)}
						>
							<div class="flex items-center gap-1.5">
								<span
									class="shrink-0 rounded px-1 py-0.5 text-[10px] leading-none font-medium uppercase {sourceBadge[
										entry.source
									] ?? sourceBadge.file}"
								>
									{entry.source}
								</span>
								<span class="text-[10px] text-muted-foreground">
									{timeAgo(entry.timestamp)}
								</span>
							</div>
							<p class="mt-1 text-xs text-foreground" class:line-clamp-1={expandedId !== entry.id}>
								{entry.text}
							</p>
						</button>

						<!-- Hover actions -->
						<div
							class="flex shrink-0 items-center gap-0.5 opacity-0 transition-opacity group-hover:opacity-100"
						>
							<button
								class="{actionBtn} hover:bg-accent hover:text-foreground"
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
								class="{actionBtn} hover:bg-destructive hover:text-white"
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

		<!-- Settings + clear -->
		<div class="flex items-center justify-between text-xs text-muted-foreground">
			<div class="flex items-center gap-2">
				<span>Save</span>
				<SettingSwitch
					checked={app.settings!.history_enabled ?? true}
					onchange={(v) => app.save({ history_enabled: v })}
				/>
				{#if !app.settings!.history_enabled}
					<span class="text-[10px] text-muted-foreground/40">paused</span>
				{/if}
			</div>

			<AlertDialog.Root>
				<AlertDialog.Trigger class="shrink-0 whitespace-nowrap underline hover:text-foreground">
					Clear all
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

			<div
				class="flex items-center gap-1.5 transition-opacity {app.settings!.history_enabled
					? ''
					: 'pointer-events-none opacity-40'}"
			>
				<span>Keep</span>
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
					class="h-6 w-14 rounded-md border border-input bg-background px-1.5 text-xs"
				/>
			</div>
		</div>
	{/if}
</div>
