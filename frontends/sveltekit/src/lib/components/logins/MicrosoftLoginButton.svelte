<script lang="ts">
	import { browser } from '$app/environment';
	import IconMicrosoft from '~icons/fa6-brands/microsoft';
	import { AUTH_SERVICE_MICROSOFT_LOGIN_URL } from '$lib/consts';
	let returnUrl: string;
    let microsoftLoginUrl: string = AUTH_SERVICE_MICROSOFT_LOGIN_URL;
	$: if (browser) {
		returnUrl = encodeURIComponent(window.location.href);
		microsoftLoginUrl = `${AUTH_SERVICE_MICROSOFT_LOGIN_URL}?return_url=${returnUrl}`;
	}
</script>

    <div class="flex justify-center">
        <a class="btn variant-filled w-36" href={microsoftLoginUrl}>
            <IconMicrosoft />
            <span class="ml-2 hidden md:inline-block">Microsoft</span>
        </a>
    </div>
