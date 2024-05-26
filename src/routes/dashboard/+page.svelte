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
		const check = document.getElementById('check');
		const check_man = document.getElementById('check-man');
		const check_blc = document.getElementById('check-blc');
		const check_ser = document.getElementById('check-ser');
		const blk = document.getElementById('blk');
		const web = document.getElementById('web');
		const refresh = document.getElementById('refresh');

		if (
			spinner === null ||
			spinner_msg === null ||
			holder === null ||
			button === null ||
			check === null ||
			check_blc === null ||
			check_ser === null ||
			check_man === null ||
			web === null ||
			blk === null ||
			refresh === null
		) {
			console.log('How did you get here?');
			return;
		}

		button.classList.add('hidden');

		const selected = await open({
			directory: true,
			multiple: false
		});

        holder.classList.remove('hidden');
        holder.classList.add('flex');

		if (selected === null) {
			spinner.classList.add('hidden');
			spinner_msg.innerHTML = 'Problem selecting a file';
		}

		let manifest_path;
		try {
			manifest_path = /** @type {string} */ (await invoke('create_manifest', { path: selected }));
		} catch (err) {
			spinner_msg.innerHTML = 'Error occured creating the manifest';
			spinner.classList.add('hidden');
			console.log(err);
			return;
		}
		await sleep(0.5);
		check.classList.remove('hidden');
		spinner_msg.innerHTML = 'Waiting for signature';

		let document_hash;
		try {
			document_hash = /** @type {string} */ (await invoke('sign', { hashJson: manifest_path }));
		} catch (err) {
			spinner_msg.innerHTML = 'Error occured signing the document';
			spinner.classList.add('hidden');
			console.log(err);
			return;
		}
		await sleep(0.5);
		check_man.classList.remove('hidden');
		spinner_msg.innerHTML = 'Sending to Blockchain';

		const blockchain_address = /** @type {string} */ (
			await invoke('blockchain', { hashedManifest: document_hash })
		);
		await sleep(0.5);

		if (blockchain_address === '') {
			spinner_msg.innerHTML = 'Failed to send to the Blockchain';
			spinner.classList.add('hidden');
			return;
		}

		check_blc.classList.remove('hidden');
		spinner_msg.innerHTML = 'Sending to the archive';
		blk.setAttribute(
			'href',
			`https://www.okx.com/pt-pt/web3/explorer/amoy/tx/0x${blockchain_address}/log`
		);
		blk.classList.remove('hidden');

		try {
			await invoke('server', { path: selected, address: blockchain_address });
		} catch (err) {
			spinner_msg.innerHTML = 'Error occured when sending files to the server';
			spinner.classList.add('hidden');
			console.log(err);
			return;
		}

		await sleep(0.5);
		web.setAttribute('href', 'http://localhost:3000/dashboard');
		web.classList.remove('hidden');
		check_ser.classList.remove('hidden');
		refresh.classList.remove('hidden');
		spinner.classList.add('hidden');
		spinner_msg.innerHTML = 'Done!';
	}
</script>

<div class="ms-52 h-full w-full px-5 py-10">
	<div class="flex w-full items-center justify-between" id="button">
		<h1 class="w-52 text-white">Choose a folder to upload</h1>
		<Button class="w-64" on:click={upload}>Upload files</Button>
	</div>
	<div class="flex-col items-center justify-center py-10">
		<div class="hidden items-center justify-center gap-2" id="holder">
			<Spinner color="white" class="h-6 animate-spin fill-white text-gray-600" id="spinner" />
			<h1 class="text-xl font-semibold text-white" id="spinner-msg">Creating Manifest</h1>
		</div>
	</div>
	<div class="flex w-full justify-between rounded-xl border border-white p-5">
		<ul class="flex flex-col text-white">
			<li class="text-xl font-bold">Task</li>
			<li>Creating Manifest</li>
			<li>Signing Manifest</li>
			<li>Sending to Blockchain</li>
			<li>Sending to Server</li>
		</ul>
		<ul class="flex flex-col text-end font-semibold text-white">
			<li class="text-xl font-bold">Status</li>
			<li class="hidden" id="check">✅</li>
			<li class="hidden" id="check-man">✅</li>
			<li class="hidden" id="check-blc">✅</li>
			<li class="hidden" id="check-ser">✅</li>
		</ul>
	</div>
	<div class="mt-5 flex w-full justify-center gap-2">
		<a href="/" target="_blank" id="blk" class="hidden">
			<Button class="">See on the Blockchain</Button>
		</a>
		<a href="/" target="_blank" id="web" class="hidden">
			<Button>See on the Website</Button>
		</a>
	</div>
	<div class="mt-5 flex w-full justify-center gap-2">
		<Button on:click={() => window.location.reload()} id="refresh" class="hidden">
			Upload another folder
		</Button>
	</div>
</div>
