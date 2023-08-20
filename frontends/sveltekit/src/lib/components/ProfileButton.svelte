<script lang="ts">
	import User from '~icons/fe/user';
	import { browser } from '$app/environment';
	import { popup } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	const popupFeatured: PopupSettings = {
		event: 'click',
		target: 'popupFeatured',
		placement: 'bottom'
	};
	import { getContext } from 'svelte';
	import { AUTH_SERVICE_LOGOUT_URL, AUTH_SERVICE_LOGIN_URL } from '$lib/consts';
	const user = getContext('user');

	let returnUrl: string;
	let loginUrl: string;
	$: if (browser) {
		returnUrl = encodeURIComponent(window.location.href);
		loginUrl = `${AUTH_SERVICE_LOGIN_URL}?return_url=${returnUrl}`;
	}
	async function logout() {
		try {
			await fetch(AUTH_SERVICE_LOGOUT_URL, {
				method: 'POST',
				credentials: 'include'
			});
			user.set(null);
		} catch (error) {
			console.error(error);
		}
	}
</script>

{#if $user && typeof $user !== 'undefined'}
	<button id="profile" aria-label="profile" class="btn-icon btn-icon-sm" use:popup={popupFeatured}>
		<User style="font-size: 1.5em" />
	</button>
	<div class="card p-4 w-72 shadow-xl" data-popup="popupFeatured">
		<div class="space-y-4">
			<a href="/profile" aria-label="Profile">
				<figure
					class="avatar flex aspect-square text-surface-50 font-semibold justify-center items-center overflow-hidden isolate bg-surface-400-500-token w-16 rounded-full"
					data-testid="avatar"
				>
					<User class="avatar-image w-full h-full object-cover" />
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
	<a class="btn btn-sm variant-soft" href={loginUrl} aria-label="Login">
		<User class="w-5 h-5" />
		<span class="hidden md:inline-block ml-2">Login</span>
	</a>
{/if}
