<script>
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { FileSolid } from 'flowbite-svelte-icons';
	import { FolderOpenSolid } from 'flowbite-svelte-icons';
</script>

<div class="ms-52 grid h-full grid-cols-3 px-5">
	<div class="col-span-2 my-8 me-5 rounded-xl bg-zinc-800 px-8 py-5 shadow-xl shadow-zinc-900">
		<div class="flex items-center py-1 font-semibold text-white">
			<FolderOpenSolid class="me-1 h-4" />
			I am a folder
		</div>
		<div class="ms-2 flex items-center border-l border-l-white ps-2 text-white">
			<FileSolid class="me-1 h-4" />
			I am a file
		</div>
		<div class="ms-2 flex items-center border-l border-l-white py-1 ps-2 font-semibold text-white">
			<FolderOpenSolid class="me-1 h-4" />
			I am a folder
		</div>
		<div class="ms-6 flex items-center border-l border-l-white ps-2 text-white">
			<FileSolid class="me-1 h-4" />
			I am a file
		</div>
		<div class="ms-6 flex items-center border-l border-l-white ps-2 text-white">
			<FileSolid class="me-1 h-4" />
			I am a file
		</div>
		<div class="ms-6 flex items-center border-l border-l-white ps-2 text-white">
			<FileSolid class="me-1 h-4" />
			I am a file
		</div>
		<div class="ms-2 flex items-center border-l border-l-white py-1 ps-2 font-semibold text-white">
			<FolderOpenSolid class="me-1 h-4" />
			I am a folder
		</div>
		<div class="ms-6 flex items-center border-l border-l-white ps-2 text-white">
			<FileSolid class="me-1 h-4" />
			I am a file
		</div>
		<div class="ms-6 flex items-center border-l border-l-white ps-2 text-white">
			<FileSolid class="me-1 h-4" />
			I am a file
		</div>
		<div class="ms-6 flex items-center border-l border-l-white ps-2 text-white">
			<FileSolid class="me-1 h-4" />
			I am a file
		</div>
	</div>
	<div class="flex flex-col py-10">
		<Label for="fileInput" class="mb-1 text-white"e>Select file or folder to upload.</Label>
		<Input id="fileInput" type="file" webkitdirectory directory multiple class="text-white" />
		<div class="flex-grow"></div>
		<Button class="w-full">Upload files</Button>
	</div>
</div>
