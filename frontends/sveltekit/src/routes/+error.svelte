<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-10 text-center flex flex-col items-center">
		<h2 class="font-bold h2">Uh oh</h2>
		{#if $page.status === 404}
			<h3 class="text-lg">The page you're looking for doesn't exist.</h3>
		{:else if $page.error && $page.error.message}
			<h3 class="text-lg">{$page.status}: {$page.error.message}</h3>
		{:else}
			<h3 class="text-lg">Something went wrong.</h3>
		{/if}
		<a class="btn variant-ghost-surface" data-sveltekit-preload-data="hover" href="/">Go home</a>
	</div>
</div>
