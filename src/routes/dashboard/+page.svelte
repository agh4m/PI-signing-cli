<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import { invoke } from '@tauri-apps/api';
	import { Spinner } from 'flowbite-svelte';
	import { open } from '@tauri-apps/api/dialog';

	/**
	 * Sleeps for a number of seconds
	 * @param {number} time
	 * @returns {Promise<void>}
	 */
	async function sleep(time) {
		return new Promise((r) => setTimeout(r, time * 1000));
	}

	async function upload() {
		const spinner = document.getElementById('spinner');
		const spinner_msg = document.getElementById('spinner-msg');
		const holder = document.getElementById('holder');
        const button = document.getElementById('button');
		if (spinner === null || spinner_msg === null || holder === null || button === null) {
			console.log('How did you get here?');
			return;
		}
		holder.classList.remove('hidden');
		holder.classList.add('flex');
        button.classList.add('hidden');

		const selected = await open({
			directory: true,
			multiple: false
		});

		if (selected === null) {
			spinner.classList.add('hidden');
			spinner_msg.innerHTML = 'Problem selecting a file';
		}

		let manifest_path;
		try {
			manifest_path = /** @type {string} */ (await invoke('create_manifest', { path: selected }));
		} catch (err) {
			console.log(err);
		}
		await sleep(1);
		spinner_msg.innerHTML = 'Waiting for signature';

		let document_hash;
		try {
			document_hash = /** @type {string} */ (
				await invoke('sign', { hash_json: manifest_path, cmd: false })
			);
		} catch (err) {
			console.log(err);
		}
		await sleep(1);
		spinner_msg.innerHTML = 'Sending to Blockchain';

		const blockchain_address = /** @type {string} */ (
			await invoke('send_blockchain', { hashed_manifest: document_hash })
		);
		await sleep(1);
		spinner_msg.innerHTML = 'Sending to the archive';

		if (blockchain_address === '') {
			spinner_msg.innerHTML === 'Failed to send to the Blockchain';
			return;
		}

		await invoke('sign', { hash_json: manifest_path, cmd: false }).catch((err) => console.log(err));
		await sleep(1);
		spinner.classList.add('hidden');
		spinner_msg.innerHTML = 'Done';
	}
</script>

<div class="ms-52 grid h-full px-5">
	<div class="flex flex-col py-10 items-center justify-center">
        <Button class="w-64" on:click={upload} id="button">Upload files</Button>
		<div class="hidden flex-col items-center justify-center gap-2" id="holder">
			<Spinner color="white" class="h-24 animate-spin fill-white text-gray-600" id="spinner" />
			<h1 class="text-2xl font-semibold text-white" id="spinner-msg">Creating Manifest</h1>
		</div>
	</div>
</div>
