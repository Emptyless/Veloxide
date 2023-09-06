<script lang="ts">
	import { goto } from '$app/navigation';
	import { getContext } from 'svelte';
	import { browser } from '$app/environment';
    import MicrosoftLoginButton from '$lib/components/logins/MicrosoftLoginButton.svelte';
    import GoogleLoginButton from '$lib/components/logins/GoogleLoginButton.svelte';
	const user = getContext('user');
	$: if (browser && typeof $user !== 'undefined') {
		goto('/profile');
	}
</script>

{#if !($user && typeof $user !== 'undefined')}
	<div class="container mx-auto flex h-full items-center justify-center">
		<div class="text-center">
			<h3 class="h3 m-10 font-bold">Choose your authentication provider</h3>
			<div class="grid grid-cols-1 items-center justify-items-center space-y-2">
                <GoogleLoginButton />
                <MicrosoftLoginButton />
			</div>
		</div>
	</div>
{/if}
