<script lang="ts">
	import User from '~icons/fe/user';
	import { popup, Avatar } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	import { URLschema } from '$lib/utils';
	const popupNotLoggedIn: PopupSettings = {
		event: 'click',
		target: 'popupNotLoggedIn',
		placement: 'bottom'
	};
	const popupLoggedIn: PopupSettings = {
		event: 'click',
		target: 'popupLoggedIn',
		placement: 'bottom'
	};
	import { getContext } from 'svelte';
	import { AUTH_SERVICE_LOGOUT_URL } from '$lib/consts';
	import type { UserView } from '$lib/stubs/auth';
	import { goto } from '$app/navigation';
	import GoogleLoginButton from './logins/GoogleLoginButton.svelte';
	import MicrosoftLoginButton from './logins/MicrosoftLoginButton.svelte';
	const user: any | UserView = getContext('user');
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
	<!-- Button for desktop -->
	<button
		id="profile-desktop"
		aria-label="profile"
		class="btn-icon btn-icon-sm hidden md:inline-block"
		use:popup={popupLoggedIn}
	>
		{#if $user.picture && URLschema.safeParse($user.picture).success}
			<Avatar src={$user.picture} class="h-full w-full" {initials} />
		{:else}
			<User style="font-size: 1.5em" />
		{/if}
	</button>
	<!-- Button for mobile -->
	<a
		href="/profile"
		id="profile-mobile"
		aria-label="profile"
		class="btn-icon btn-icon-sm md:hidden"
	>
		{#if $user.picture && URLschema.safeParse($user.picture).success}
			<Avatar src={$user.picture} class="h-full w-full" {initials} />
		{:else}
			<User style="font-size: 1.5em" />
		{/if}
	</a>
	<!-- Popup on desktop -->
	<div class="card w-72 p-4 shadow-xl" data-popup="popupLoggedIn">
		<div class="space-y-4">
			<a href="/profile" aria-label="Profile">
				<figure
					class="avatar bg-surface-400-500-token isolate flex aspect-square w-16 items-center justify-center overflow-hidden rounded-full font-semibold text-surface-50"
					data-testid="avatar"
				>
					{#if $user.picture && URLschema.safeParse($user.picture).success}
						<Avatar src={$user.picture} class="h-full w-full" />
					{:else}
						<User class="avatar-image h-full w-full object-cover" />
					{/if}
				</figure>
			</a>
			<div>
				<p class="font-bold">{$user.name}</p>
				<p class="opacity-50">{$user.email}</p>
			</div>
			<button class="btn variant-soft w-full" on:click={logout}>Logout</button>
		</div>
	</div>
{:else}
	<!-- Button for desktop -->
	<button
		id="profile-desktop"
		aria-label="profile"
		class="btn btn-sm variant-soft hidden md:inline-block"
		use:popup={popupNotLoggedIn}
	>
		<User class="w-5 h-5 hidden md:inline-block" />
		<span class="hidden md:inline-block ml-2">Login</span>
	</button>
	<!-- Button for mobile -->
	<a
		href="/profile"
		id="profile-mobile"
		aria-label="profile"
		class="btn-icon btn-icon-sm md:hidden"
	>
		<User style="font-size: 1.5em" />
	</a>
	<!-- Popup on desktop -->
	<div class="card w-72 p-4 shadow-xl" data-popup="popupNotLoggedIn">
		<div class="grid grid-cols-1 items-center justify-items-center space-y-2">
			<span class="m-4 text-center h4 font-bold">Choose your authentication provider</span>
			<GoogleLoginButton />
			<MicrosoftLoginButton />
		</div>
	</div>
{/if}
