<script>
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { FileSolid } from 'flowbite-svelte-icons';
	import { FolderOpenSolid } from 'flowbite-svelte-icons';
</script>

<div class="ms-52 grid h-full px-5">
	<div class="flex flex-col py-10">
		<Label for="fileInput" class="mb-1 text-white"e>Select file or folder to upload.</Label>
		<Input id="fileInput" type="file" class="text-white" />
		<div class="flex-grow"></div>
		<Button class="w-full">Upload files</Button>
	</div>
</div>
