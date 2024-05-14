<script setup lang="ts">
async function login() {
    const token = document.querySelector('input').value;

    await navigateTo('/dashboard');
}
</script>

<template>
    <div
        class="flex h-[500px] w-96 flex-col items-center justify-center gap-5 rounded-xl bg-zinc-800 p-10 shadow-xl shadow-zinc-950">
        <div class="hidden w-full items-center justify-center rounded-xl border border-red-500 p-5 text-red-500"
            id="err">
            Login failed, try logging in again.
        </div>
        <h1 class="text-4xl font-bold text-white">Sign in to DiSA</h1>
        <p class="mb-6 text-white text-center font-semibold">Insert the token provided in the web app to get started.</p>
        <Input class="w-80 text-white" placeholder="Token" type="text" />
        <Button class="shadown-md h-10 w-80 bg-zinc-700 font-semibold shadow-zinc-950" @click="login">Login</Button>
    </div>
</template>
