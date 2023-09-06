<script lang="ts">
	import { browser } from '$app/environment';
	import IconGoogle from '~icons/fa6-brands/google';
	import { AUTH_SERVICE_GOOGLE_LOGIN_URL } from '$lib/consts';
	let returnUrl: string;
    let googleLoginUrl: string = AUTH_SERVICE_GOOGLE_LOGIN_URL;
	$: if (browser) {
		returnUrl = encodeURIComponent(window.location.href);
		googleLoginUrl = `${AUTH_SERVICE_GOOGLE_LOGIN_URL}?return_url=${returnUrl}`;
	}
</script>


<div class="flex justify-center">
    <a class="btn variant-filled w-36" href={googleLoginUrl}>
        <IconGoogle />
        <span class="ml-2 hidden md:inline-block">Google</span>
    </a>
</div>
