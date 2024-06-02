<script>
	import Button from './ui/button/button.svelte';
	import * as DropdownMenu from './ui/dropdown-menu';
	import { UserCircleSolid } from 'flowbite-svelte-icons';

	/**
	 *  @param {Event} e
	 */
	function logout(e) {
        e.preventDefault();
		window.location.href = '/';
	}
</script>

<div class="absolute left-0 flex h-screen w-52 flex-col gap-4 bg-zinc-900 p-5">
	<a href="/dashboard" data-sveltekit-preload-data="hover" class="w-44">
		<Button class="w-full bg-zinc-800 font-semibold shadow shadow-zinc-950">
			Upload Documents
		</Button>
	</a>
	<a href="/dashboard/verify" data-sveltekit-preload-data="hover" class="w-44">
		<Button class="w-full bg-zinc-800 font-semibold shadow shadow-zinc-950">
			Verify Authenticity
		</Button>
	</a>
	<div class="flex-grow"></div>
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class="flex items-center gap-1 text-zinc-300">
			<UserCircleSolid class="h-6" />
			Account
		</DropdownMenu.Trigger>
		<DropdownMenu.Content class="w-48 border-black bg-black text-white">
			<DropdownMenu.Group>
				<DropdownMenu.Item on:click={logout}>Log out</DropdownMenu.Item>
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
</div>
