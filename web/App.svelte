<script lang="ts">
	const github = 'https://github.com/josephembrey/wisp';
	const download = `${github}/releases/latest`;
</script>

<div class="flex min-h-screen flex-col items-center justify-center px-6">
	<main class="flex max-w-lg flex-col items-center gap-8 text-center">
		<h1 class="text-4xl font-bold tracking-tight">wisp</h1>
		<p class="text-lg text-muted-foreground">
			Local, private speech-to-text. Push a key, speak, get text. Powered by
			<a
				href="https://github.com/ggerganov/whisper.cpp"
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
				class="inline-flex items-center gap-2 rounded-lg bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				Download
			</a>
			<a
				href={github}
				class="inline-flex items-center gap-2 rounded-lg border border-border px-5 py-2.5 text-sm font-medium text-foreground hover:bg-secondary"
			>
				GitHub
			</a>
		</div>

		<p class="text-xs text-muted-foreground">Windows &middot; macOS &middot; Linux</p>
	</main>
</div>
