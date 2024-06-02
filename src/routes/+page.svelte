<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { invoke } from '@tauri-apps/api';

	import { listen } from '@tauri-apps/api/event';

	let token = '';

	async function login() {
		let err = document.getElementById('err');
		if (err === null) {
			alert('How did you get here?');
			return;
		}
		if (err.classList.contains('flex')) {
			err.classList.remove('flex');
			err.classList.add('hidden');
		}

		if (token === '' || token === undefined) {
			err.classList.remove('hidden');
			err.classList.add('flex');
			err.innerHTML = 'Please enter a token.';
			return;
		}

		await invoke('login_user', { token: token })
			.then((token) => {
				if (token === 'Could not login') {
					err.classList.remove('hidden');
					err.classList.add('flex');
					err.innerHTML = 'Login failed, try logging in again.';
					return;
				}
				localStorage.setItem('token', token);
				window.location.href = '/dashboard';
			})
			.catch(() => {
				err.classList.remove('hidden');
				err.classList.add('flex');
				err.innerHTML = 'Login failed, try logging in again.';
			});
	}
</script>

<div
	class="flex h-[500px] w-96 flex-col items-center justify-center gap-5 rounded-xl bg-zinc-800 p-10 shadow-xl shadow-zinc-950"
>
	<div
		class="hidden w-full items-center justify-center rounded-xl border border-red-500 p-5 text-red-500"
		id="err"
	>
		Login failed, try logging in again.
	</div>
	<h1 class="text-4xl font-bold text-white">Sign in to DiSA</h1>
	<p class="mb-8 text-center text-white">
		To start uploading your files login using the token you get from the webapp.
	</p>
	<form on:submit={login} class="flex flex-col gap-6">
		<Input class="w-80 text-white" bind:value={token} placeholder="Token" type="text" />
		<Button class="shadown-md h-10 w-80 bg-zinc-700 font-semibold shadow-zinc-950" on:click={login}>
			Login
		</Button>
	</form>
</div>
