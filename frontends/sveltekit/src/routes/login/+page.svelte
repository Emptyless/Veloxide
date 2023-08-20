<script lang="ts">
	import { goto } from '$app/navigation';
	import IconGoogle from '~icons/fa6-brands/google';
	import { getContext } from 'svelte';
	import { browser } from '$app/environment';
	import { AUTH_SERVICE_LOGIN_URL } from '$lib/consts';
	const user = getContext('user');
	$: if (browser && typeof $user !== 'undefined') {
		goto('/profile');
	}

	let returnUrl: string;
	let loginUrl: string;
	$: if (browser) {
		returnUrl = encodeURIComponent(window.location.href);
		loginUrl = `${AUTH_SERVICE_LOGIN_URL}?return_url=${returnUrl}`;
	}
</script>

{#if !($user && typeof $user !== 'undefined')}
	<div class="container h-full mx-auto justify-center items-center flex">
		<div class="text-center">
			<h3 class="font-bold h3 m-10">Choose your authentication provider</h3>
			<a class="btn variant-filled" href={loginUrl}>
				<IconGoogle />
				<span class="hidden md:inline-block ml-2">Google</span>
			</a>
		</div>
	</div>
{/if}
