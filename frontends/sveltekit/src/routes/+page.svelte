<script lang="ts">
	import GetBankAccount from '$lib/components/GetBankAccount.svelte';
	import GreeterButton from '$lib/components/GreeterButton.svelte';
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-10 text-center flex flex-col items-center">
		<h2 class="h2">gRPC Connection Tester</h2>
		<GreeterButton />
		<GetBankAccount />
	</div>
</div>
