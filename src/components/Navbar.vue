<script setup lang="ts">
import { useRoute } from 'vue-router';
import { appWindow } from "@tauri-apps/api/window"
import Close from '~icons/fe/close'
import Min from '~icons/fe/minus'
import Max from '~icons/material-symbols/rectangle-outline-rounded'

const route = useRoute();

const run = (command: string) => {
    if (command === 'close') {
        appWindow.close();
    }
    if (command === 'min') {
        appWindow.minimize();
    }
    if (command === 'max') {
        appWindow.toggleMaximize();
    }
}

</script>

<template>
    <div class="flex text-light">
        <div class="titlebar z-10 h-10 top-0 left-0 fixed w-full" data-tauri-drag-region>
        </div>

        <div class="w-full">
            <div class="flex">
                <Transition name="fade" mode="out-in">
                    <div :key="route.path" class="flex items-baseline px-4 space-x-4">
                        <h1 class="text-2xl py-2 text-accent font-bold">{{ route.meta.title || route.name }}</h1>
                        <p class="text-zinc-200">{{ route.meta.description || "" }}</p>
                    </div>
                </Transition>
                <div class="flex ml-auto z-20">
                    <button class="w-10 h-full flex items-center justify-center hover:bg-zinc-700" @click="run('min')">
                        <Min class="w-4 h-4 -ml-0.5 -mt-0.5"></Min>
                    </button>
                    <button class="w-10 h-full flex items-center justify-center hover:bg-zinc-700" @click="run('max')">
                        <Max class="w-4 h-4 -mt-0.5"></Max>
                    </button>
                    <button class="w-10 h-full flex items-center justify-center hover:bg-accent" @click="run('close')">
                        <Close class="w-5 h-5 -ml-0.5 -mt-0.5"></Close>
                    </button>
                </div>
            </div>
            <hr class="border-zinc-700">
        </div>
    </div>
</template>
<style>
.titlebar {
    user-select: none;
}
</style>