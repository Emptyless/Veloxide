<script lang="ts">
	import { getContext } from 'svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	const user = getContext('user');
	import { browser } from '$app/environment';
	import { AUTH_SERVICE_LOGOUT_URL } from '$lib/consts';
	onMount(() => {
		if ($user === null || typeof $user === 'undefined') {
			goto('/login');
		}
	});
	$: if (browser && typeof $user === 'undefined') {
		goto('/login');
	}
	async function logout() {
		try {
			await fetch(AUTH_SERVICE_LOGOUT_URL, {
				method: 'POST',
				credentials: 'include'
			});
			user.set(undefined);
		} catch (error) {
			console.error(error);
		}
	}
</script>

{#if $user && typeof $user !== 'undefined'}
	<div class="container h-full mx-auto justify-center items-center flex">
		<div class="text-center">
			<h2 class="font-bold h2 m-10">Your profile</h2>
			<p class="font-bold">{$user.name}</p>
			<p class="opacity-50">{$user.email}</p>
			<div class="grid grid-cols-1 row-gap-4">
				<div class="m-2">
					<button class="btn variant-filled" on:click={logout}>Logout</button>
				</div>
			</div>
		</div>
	</div>
{/if}
