<script lang="ts">
	import User from '~icons/fe/user';
	import { getContext } from 'svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import type { UserView } from '$lib/stubs/auth';
	import { URLschema } from '$lib/utils';
	const user: any | UserView = getContext('user');
	import { browser } from '$app/environment';
	import { AUTH_SERVICE_LOGOUT_URL, AUTH_TOKEN_COOKIE_DOMAIN } from '$lib/consts';
	import { Avatar } from '@skeletonlabs/skeleton';

	onMount(() => {
		if ($user === null || typeof $user === 'undefined') {
			goto('/login');
		}
	});
	$: if (browser && typeof $user === 'undefined') {
		goto('/login');
	}

	let initials: string;
	$: if ($user && typeof $user !== 'undefined') {
		initials = `${$user.givenName.charAt(0).toUpperCase()}${$user.familyName
			.charAt(0)
			.toUpperCase()}`;
	}

	async function logout(): Promise<void> {
		try {
			await fetch(AUTH_SERVICE_LOGOUT_URL, {
				method: 'POST',
				credentials: 'include'
			});
			user.set(undefined);
			goto('/');
		} catch (error) {
			user.set(undefined);
			console.error(error);
		}
	}
</script>

{#if $user && typeof $user !== 'undefined'}
	<div class="container h-full mx-auto flex flex-col items-center justify-center">
		<div class="text-center">
			<h2 class="font-bold text-2xl m-10">Your profile</h2>

			<div class="w-24 h-24 mb-4 rounded-full overflow-hidden mx-auto">
				{#if $user.picture && URLschema.safeParse($user.picture).success}
					<Avatar src={$user.picture} class="w-full h-full object-cover" {initials} />
				{:else}
					<User />
				{/if}
			</div>

			<p class="font-bold">{$user.name}</p>
			<p class="text-opacity-50">{$user.email}</p>
			<div class="grid grid-cols-1 gap-y-4 mt-4">
				<div class="m-2">
					<button class="btn variant-filled" on:click={logout}>Logout</button>
				</div>
			</div>
		</div>
	</div>
{/if}
