<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';
	import {
		getModels,
		downloadModel,
		deleteModel,
		isFirstRun,
		onDownloadProgress,
		type ModelInfo,
		type Settings,
		type DownloadProgress
	} from '$lib/tauri';
	import { toast } from 'svelte-sonner';

	let {
		settings,
		onsave,
		ondownloadchange
	}: {
		settings: Settings;
		onsave: (updates: Partial<Settings>) => void;
		ondownloadchange?: (downloading: boolean) => void;
	} = $props();

	let models: ModelInfo[] = $state([]);
	let downloading: string | null = $state(null);
	let progress: DownloadProgress | null = $state(null);

	let selectedModel = $derived(models.find((m) => m.name === settings.model));

	async function load() {
		models = await getModels();

		if ((await isFirstRun()) && !downloading) {
			handleDownload('base');
		}
	}

	async function handleDownload(name: string) {
		downloading = name;
		progress = null;
		ondownloadchange?.(true);
		try {
			await downloadModel(name);
			models = await getModels();
		} catch (e) {
			toast.error(`Failed to download model: ${e}`);
		} finally {
			downloading = null;
			progress = null;
			ondownloadchange?.(false);
		}
	}

	async function handleDelete(name: string) {
		try {
			await deleteModel(name);
			models = await getModels();
		} catch (e) {
			toast.error(`Failed to delete model: ${e}`);
		}
	}

	$effect(() => {
		load();
		const unsub = onDownloadProgress((p) => (progress = p));
		return () => {
			unsub.then((fn) => fn());
		};
	});
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
						onclick={() => handleDownload(selectedModel.name)}
						disabled={downloading !== null}
					>
						{downloading === selectedModel.name ? 'Downloading...' : 'Download'}
					</Button>
				{:else}
					<Button
						size="sm"
						variant="outline"
						onclick={() => handleDelete(selectedModel.name)}
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
