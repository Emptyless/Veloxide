<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	import '$lib/styles/veloxide-theme.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of the app wide CSS should be put in this file
	import '../app.postcss';

	import { Toast, AppShell } from '@skeletonlabs/skeleton';
	import Analytics from '$lib/components/Analytics.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import FooterBar from '$lib/components/FooterBar.svelte';
</script>

<Toast />
<Analytics />
<AppShell slotSidebarLeft="bg-surface-500/5 w-56 p-4">
	<svelte:fragment slot="header">
		<Navbar />
	</svelte:fragment>
	<div class="invisible pb-4" />
	<slot />
	<svelte:fragment slot="pageFooter">
		<div class="pt-10">
			<!-- This div needs to stay to prevent the button overlapping the footer -->
			<FooterBar />
		</div>
	</svelte:fragment>
</AppShell>
