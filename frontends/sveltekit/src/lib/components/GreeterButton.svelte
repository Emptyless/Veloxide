<script lang="ts">
	import { toastStore } from '@skeletonlabs/skeleton';
	import type { ToastSettings } from '@skeletonlabs/skeleton';
	import { greeterClient } from '$lib/greeterClient';
	let name: string;
	async function sayHello(): Promise<void> {
		const request = { name };
		const response = await greeterClient.sayHello(request);
		console.log(response.response);
		const t: ToastSettings = {
			message: response.response.message
		};
		toastStore.trigger(t);
	}
</script>

<div class="grid grid-cols-1 row-gap-4">
	<div class="m-2">
		<input
			class="input"
			title="Type your name"
			type="text"
			placeholder="Type your name"
			bind:value={name}
		/>
		<button on:click={sayHello} class="btn variant-filled"> Say Hello </button>
	</div>
</div>
