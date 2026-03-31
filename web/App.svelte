<script lang="ts">
	const github = 'https://github.com/josephembrey/wisp';
	const download = `${github}/releases/latest`;

	// Interactive demo — track physical key state directly
	type DemoState = 'idle' | 'recording' | 'processing' | 'copied' | 'cancelled' | 'mode';
	type OutputMode = 'clipboard' | 'type';
	let status: DemoState = $state('idle');
	let display: DemoState = $state('idle');
	let outputMode: OutputMode = $state('clipboard');
	let inputText = $state('');
	let fakeClipboard = '';
	let ctrlHeld = $state(false);
	let altHeld = $state(false);
	let qHeld = $state(false);
	let recording = false;
	let recordStart = 0;
	let timeout: ReturnType<typeof setTimeout> | undefined;
	let typeInterval: ReturnType<typeof setInterval> | undefined;

	const demoText = 'hello, this is a demo transcription';

	function typewrite(text: string) {
		clearInterval(typeInterval);
		let i = 0;
		typeInterval = setInterval(() => {
			i++;
			inputText = text.slice(0, i);
			if (i >= text.length) clearInterval(typeInterval);
		}, 10);
	}

	$effect(() => {
		if (status !== 'idle') display = status;
	});

	function comboDown() {
		return altHeld && qHeld;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.repeat) return;
		if (e.code === 'ControlLeft' || e.code === 'ControlRight') ctrlHeld = true;
		if (e.code === 'AltLeft' || e.code === 'AltRight') {
			altHeld = true;
			e.preventDefault();
		}
		if (e.code === 'KeyQ') {
			qHeld = true;
			e.preventDefault();
		}

		// Ctrl+V = paste from fake clipboard
		if (ctrlHeld && e.code === 'KeyV' && fakeClipboard) {
			e.preventDefault();
			inputText = fakeClipboard;
			return;
		}

		if (!comboDown()) return;

		// Ctrl+Alt+Q = toggle output mode
		if (ctrlHeld) {
			clearTimeout(timeout);
			outputMode = outputMode === 'clipboard' ? 'type' : 'clipboard';
			status = 'mode';
			timeout = setTimeout(() => (status = 'idle'), 1200);
			return;
		}

		// Alt+Q = start recording
		if (!recording) {
			recording = true;
			recordStart = Date.now();
			clearTimeout(timeout);
			clearInterval(typeInterval);
			inputText = '';
			fakeClipboard = '';
			status = 'recording';
		}
	}

	function handleBlur() {
		ctrlHeld = false;
		altHeld = false;
		qHeld = false;
		if (recording) {
			recording = false;
			clearTimeout(timeout);
			status = 'cancelled';
			timeout = setTimeout(() => (status = 'idle'), 1000);
		}
	}

	function handleKeyup(e: KeyboardEvent) {
		if (e.code === 'ControlLeft' || e.code === 'ControlRight') ctrlHeld = false;
		if (e.code === 'AltLeft' || e.code === 'AltRight') altHeld = false;
		if (e.code === 'KeyQ') qHeld = false;

		if (recording && !comboDown()) {
			recording = false;
			clearTimeout(timeout);
			if (Date.now() - recordStart < 500) {
				status = 'cancelled';
				timeout = setTimeout(() => (status = 'idle'), 1000);
				return;
			}
			status = 'processing';
			timeout = setTimeout(() => {
				status = 'copied';
				if (outputMode === 'type') {
					typewrite(demoText);
				} else {
					fakeClipboard = demoText;
				}
				timeout = setTimeout(() => (status = 'idle'), 1000);
			}, 600);
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} onkeyup={handleKeyup} onblur={handleBlur} />

<div class="flex min-h-screen flex-col items-center justify-center px-6">
	<!-- Overlay pill — fixed top-right -->
	<div
		class="fixed top-4 right-4 flex flex-col items-end gap-1 transition-opacity duration-150"
		class:opacity-0={status === 'idle'}
	>
		<div class="overlay-pill flex w-fit items-center gap-2 rounded-full px-3 py-1.5 shadow-lg">
			{#if display === 'recording'}
				<span class="relative flex h-2.5 w-2.5 shrink-0">
					<span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-red-500"
					></span>
					<span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-red-500"></span>
				</span>
				<span class="overlay-label text-white">Recording</span>
			{:else if display === 'processing'}
				<svg
					class="h-3.5 w-3.5 shrink-0 animate-spin text-amber-400"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
				>
					<path d="M21 12a9 9 0 1 1-6.219-8.56" />
				</svg>
				<span class="overlay-label text-white">Processing</span>
			{:else if display === 'cancelled'}
				<svg
					class="h-3 w-3 shrink-0 text-amber-400"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<line x1="18" y1="6" x2="6" y2="18" />
					<line x1="6" y1="6" x2="18" y2="18" />
				</svg>
				<span class="overlay-label text-amber-400">Cancelled</span>
			{:else if display === 'copied'}
				<svg
					class="h-3 w-3 shrink-0 text-green-400"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
					<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
					<path d="m9 14 2 2 4-4" />
				</svg>
				<span class="overlay-label text-green-400">Copied</span>
			{:else if display === 'mode'}
				{#if outputMode === 'clipboard'}
					<svg
						class="h-3 w-3 shrink-0 text-white"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
						<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
					</svg>
					<span class="overlay-label text-white">Clipboard</span>
				{:else}
					<svg
						class="h-3 w-3 shrink-0 text-white"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1" />
						<path d="M7 22h1a4 4 0 0 0 4-4v-1" />
						<path d="M7 2h1a4 4 0 0 1 4 4v1" />
					</svg>
					<span class="overlay-label text-white">Cursor</span>
				{/if}
			{/if}
		</div>
	</div>

	<main class="flex max-w-lg flex-col items-center gap-8 text-center">
		<div class="flex flex-col items-center gap-5">
			<div class="flex items-center gap-4">
				<h1 class="text-4xl font-bold tracking-tight">wisp</h1>
				<div class="flex items-center gap-1">
					<kbd class="kbd kbd-sm kbd-dim {ctrlHeld ? 'kbd-active' : ''}">Ctrl</kbd>
					<span class="text-[10px] text-muted-foreground/20">+</span>
					<kbd class="kbd kbd-sm {altHeld ? 'kbd-active' : ''}">Alt</kbd>
					<span class="text-[10px] text-muted-foreground/40">+</span>
					<kbd class="kbd kbd-sm {qHeld ? 'kbd-active' : ''}">Q</kbd>
				</div>
			</div>
			<input type="text" bind:value={inputText} placeholder="..." class="demo-input" />
		</div>

		<p class="text-lg text-muted-foreground">
			Local, private speech-to-text. Push a key, speak, get text. Powered by
			<a
				href="https://github.com/ggerganov/whisper.cpp"
				rel="noopener noreferrer"
				class="underline underline-offset-4 hover:text-foreground">Whisper</a
			>.
		</p>

		<ul class="flex flex-col gap-2 text-sm text-muted-foreground">
			<li>Runs entirely on your machine &mdash; no cloud, no API keys</li>
			<li>GPU accelerated via Vulkan / Metal</li>
			<li>Works anywhere you can type</li>
		</ul>

		<div class="flex gap-3">
			<a
				href={download}
				rel="noopener noreferrer"
				class="inline-flex items-center gap-2 rounded-lg bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				Download
			</a>
			<a
				href={github}
				rel="noopener noreferrer"
				class="inline-flex items-center gap-2 rounded-lg border border-border px-5 py-2.5 text-sm font-medium text-foreground hover:bg-secondary"
			>
				GitHub
			</a>
		</div>

		<p class="text-xs text-muted-foreground">Windows &middot; macOS &middot; Linux</p>
	</main>
</div>

<style>
	.overlay-pill {
		background: rgba(0, 0, 0, 0.6);
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.overlay-label {
		font-size: 0.75rem;
		font-weight: 500;
		white-space: nowrap;
	}

	.kbd {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 2rem;
		padding: 0.25rem 0.75rem;
		border-radius: 0.375rem;
		border: 1px solid var(--border);
		background: var(--muted);
		color: var(--muted-foreground);
		font-family: inherit;
		font-size: 0.875rem;
		font-weight: 500;
		transition: all 150ms;
	}

	.kbd-sm {
		min-width: 1.5rem;
		padding: 0.2rem 0.5rem;
		font-size: 0.7rem;
	}

	.demo-input {
		width: 280px;
		height: 2rem;
		padding: 0 0.75rem;
		border-radius: 0.375rem;
		border: 1px solid var(--border);
		background: var(--background);
		color: var(--foreground);
		font-family: inherit;
		font-size: 0.75rem;
		text-align: left;
		outline: none;
	}

	.demo-input::placeholder {
		color: var(--muted-foreground);
		opacity: 0.4;
	}

	.kbd-dim {
		opacity: 0.4;
	}

	.kbd-dim.kbd-active {
		opacity: 1;
	}

	.kbd-active {
		border-color: var(--primary);
		color: var(--primary);
		box-shadow: 0 0 8px oklch(0.7 0.15 250 / 0.3);
	}
</style>
