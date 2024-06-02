<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import { invoke } from '@tauri-apps/api';
	import { open } from '@tauri-apps/api/dialog';

	async function sig_verify() {
		const verification_text = document.getElementById('ver_text');

		if (verification_text === null) {
			console.log('Again how? :(');
			return;
		}

		const selected = await open({
			directory: false,
			multiple: false
		});

		let result;
		try {
			result = /** @type {number} */ (await invoke('verify', { path: selected }));
		} catch (err) {
			console.log(err);
			return;
		}

		if (result === 0) {
			verification_text.innerHTML = 'The signature is valid';
		} else {
			verification_text.innerHTML = 'The signature is invalid';
		}
	}

	async function verify_manifest() {
		const verification_text = document.getElementById('fver_text');
		const blockchain_path = /** @type {HTMLInputElement | null} */ (
			document.getElementById('blch_input')
		);

		if (verification_text === null || blockchain_path === null) {
			console.log('Again how? :(');
			return;
		}

		const selected = await open({
			directory: true,
			multiple: false
		});

		if (blockchain_path.value === '' || !blockchain_path.value.startsWith('0x')) {
			verification_text.innerHTML =
				'Please provide a valid blockchain address, it should start with 0x';
			return;
		}

		let manifest_path;
		try {
			manifest_path = /** @type {string} */ (await invoke('create_manifest', { path: selected }));
		} catch (err) {
			verification_text.innerHTML = 'There was an error recreating the manifest';
			console.log(err);
			return;
		}

		let result;
		try {
			result = /** @type {Boolean} */ (
				await invoke('verify_blockchain', {
					manifestPath: manifest_path,
					blockchainAddress: blockchain_path.value
				})
			);
		} catch (err) {
			verification_text.innerHTML = 'There was an error fetching the hash from the blockchain';
			console.log(err);
			return;
		}

		console.log(result);
		console.log(typeof result);
		if (result === true) {
			console.log(result);
			verification_text.innerHTML =
				'The manifest hash corresponds to the contents of the blockchain';
			return;
		}
		verification_text.innerHTML =
			'The blockchain content does not match the manifest hash (files might have been altered)';
	}

	function reset() {
		const sig_text = document.getElementById('ver_text');
		const block_text = document.getElementById('fver_text');
		const blockchain_path = /** @type {HTMLInputElement | null} */ (
			document.getElementById('blch_input')
		);

		if (sig_text === null || block_text === null || blockchain_path === null) {
			return;
		}

		sig_text.innerHTML = 'The signature has not been verified';
		block_text.innerHTML = 'Files have not been verified';
		blockchain_path.value = '';
	}
</script>

<div class="ms-52 flex h-full w-full flex-col p-10">
	<div class="flex w-full items-center justify-between" id="button">
		<h1 class="w-52 text-white">Choose a signature to verify</h1>
		<Button class="w-64" on:click={sig_verify}>Choose Signature</Button>
	</div>
	<div class="my-5 text-center font-semibold text-white">
		<p id="ver_text">The signature has not been verified</p>
	</div>
	<Separator />
	<form action="" class="mt-5">
		<Label for="blch_input" class="text-white">
			Blockchain address (required for file verification)
		</Label>
		<Input id="blch_input" placeholder="Blockchain address" class="text-white" />
	</form>
	<div class="mt-5 flex w-full items-center justify-between" id="button">
		<h1 class="w-52 text-white">Choose a the files to verify</h1>
		<Button class="w-64" on:click={verify_manifest}>Choose Files</Button>
	</div>
	<div class="my-5 text-center font-semibold text-white">
		<p id="fver_text">Files have not been verified</p>
	</div>
	<div class="flex-grow"></div>
	<div class="mx-auto">
		<Button class="w-64" on:click={reset}>Verify a new set of documents</Button>
	</div>
</div>
