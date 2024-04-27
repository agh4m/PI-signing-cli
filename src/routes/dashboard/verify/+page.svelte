<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
</script>

<div class="ms-52 flex h-full w-full flex-col p-10">
	<Label for="link" class="mb-2 ms-1 text-white">Select manifest to verify</Label>
	<Input class="w-full text-white" type="file" id="link" placeholder="Download link" />
	<div class="flex-grow"></div>
	<Button class="w-72 me-0 ms-auto" disabled>Verify</Button>
</div>
