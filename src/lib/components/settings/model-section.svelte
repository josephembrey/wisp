<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';
	import type { ModelInfo, Settings, DownloadProgress } from '$lib/tauri';

	let {
		settings,
		models,
		downloading = null,
		progress = null,
		onsave,
		ondownload,
		ondelete
	}: {
		settings: Settings;
		models: ModelInfo[];
		downloading?: string | null;
		progress?: DownloadProgress | null;
		onsave: (updates: Partial<Settings>) => void;
		ondownload: (name: string) => void;
		ondelete: (name: string) => void;
	} = $props();

	let selectedModel = $derived(models.find((m) => m.name === settings.model));
</script>

<div class="flex flex-col gap-1.5">
	<span class="text-xs font-medium text-muted-foreground">Model</span>
	<div class="flex flex-col gap-1.5">
		<div class="flex items-center gap-2">
			<Select.Root
				type="single"
				value={settings.model}
				onValueChange={(v) => {
					if (v) onsave({ model: v });
				}}
			>
				<Select.Trigger class="flex-1">
					{selectedModel?.name ?? settings.model}
				</Select.Trigger>
				<Select.Content>
					{#each models as model (model.name)}
						<Select.Item value={model.name}>
							{model.name}
							<span class="ml-auto text-xs text-muted-foreground">
								{model.size_mb} MB
								{#if model.downloaded}&check;{/if}
							</span>
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>

			{#if selectedModel}
				{#if !selectedModel.downloaded}
					<Button
						size="sm"
						onclick={() => ondownload(selectedModel.name)}
						disabled={downloading !== null}
					>
						{downloading === selectedModel.name ? 'Downloading...' : 'Download'}
					</Button>
				{:else}
					<Button
						size="sm"
						variant="outline"
						onclick={() => ondelete(selectedModel.name)}
						class="h-8 w-8 shrink-0 p-0"
						aria-label="Delete model"
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
							><polyline points="3 6 5 6 21 6" /><path
								d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
							/></svg
						>
					</Button>
				{/if}
			{/if}
		</div>

		{#if downloading && progress && progress.total > 0}
			{@const pct = Math.round((progress.downloaded / progress.total) * 100)}
			<div class="flex items-center gap-2">
				<Progress value={pct} class="flex-1" />
				<span class="text-xs text-muted-foreground tabular-nums">{pct}%</span>
			</div>
		{/if}
	</div>
</div>
