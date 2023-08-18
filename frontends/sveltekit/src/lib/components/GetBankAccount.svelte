<script lang="ts">
	import { toastStore } from '@skeletonlabs/skeleton';
	import type { ToastSettings } from '@skeletonlabs/skeleton';
	import { bankAccountClient } from '$lib/bankAccountClient';
	import type { GetBankAccountRequest } from '$lib/stubs/bank_account_service';

	let accountId = '123'; // Using camelCase as it's more conventional in JavaScript
	async function getBankAccount(): Promise<void> {
		try {
			const request: GetBankAccountRequest = { id: accountId };
			const response = await bankAccountClient.getBankAccount(request);
			if (!response.response.accountView?.balance) {
				throw new Error('Account balance not found');
			}
			const t: ToastSettings = {
				message: response.response.accountView.balance.toString()
			};
			toastStore.trigger(t);
		} catch (error) {
			console.error('An error occurred while fetching the bank account:', error);
			toastStore.trigger({
				message: 'Failed to fetch the bank account.'
			});
		}
	}
</script>

<div class="m-2">
	<input
		class="input"
		title="Type your account_id"
		type="text"
		placeholder="Type your account_id"
		bind:value={accountId}
	/>
	<button on:click={getBankAccount} class="btn variant-filled"> Get bank account </button>
</div>
