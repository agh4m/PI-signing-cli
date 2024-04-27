<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	let value = '';
</script>

<div
	class="flex h-[500px] w-96 flex-col items-center justify-center gap-5 rounded-xl bg-zinc-800 p-10 shadow-xl shadow-zinc-950"
>
	<h1 class="text-4xl font-bold text-white">Sign in to DiSA</h1>
	<p class="mb-8 text-white">Start uploading your files today.</p>
	<Input class="w-80 text-white" bind:value placeholder="Email" type="text" />
	<Input class="w-80 text-white" bind:value type="password" placeholder="Password" />
	<a href="/dashboard" data-sveltekit-preload-data="hover">
		<Button class="shadown-md h-10 w-80 bg-zinc-700 font-semibold shadow-zinc-950">Login</Button>
	</a>
</div>
