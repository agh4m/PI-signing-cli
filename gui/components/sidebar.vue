<template>
    <div class="absolute left-0 flex h-screen w-52 flex-col gap-4 bg-zinc-900 p-5">
        <a :href="dashboard" class="w-44">
            <Button class="w-full bg-zinc-800 font-semibold shadow shadow-zinc-950">
                Upload Documents
            </Button>
        </a>
        <a :href="dashboard/verify" class="w-44">
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
                    <DropdownMenu.Item>My files</DropdownMenu.Item>
                    <DropdownMenu.Separator class="mx-auto w-[90%] text-zinc-300" />
                    <DropdownMenu.Item>Log out</DropdownMenu.Item>
                </DropdownMenu.Group>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
</template>
