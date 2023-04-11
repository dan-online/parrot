<script setup lang="ts">
import Home from "~icons/fe/home";
import Search from "~icons/fe/search";
import Gear from "~icons/fe/gear";
import Download from '~icons/fe/download'

import { version } from "../../package.json"

const pages = [
    {
        name: "Home",
        path: "/",
        icon: Home,
    },
    {
        name: "Search",
        path: "/search",
        icon: Search,
    },

    {
        name: "Installed",
        path: "/installed",
        icon: Download,
    },
    // {
    //     name: "Settings",
    //     path: "/settings",
    //     icon: Gear,
    // },

]

</script>
<template>
    <div class="bg-dark w-48 h-full min-h-screen border-r border-zinc-700">
        <div class="flex items-baseline pt-2">
            <h1 class="text-accent text-2xl font-semibold ml-4">Parrot</h1>
            <span class="text-zinc-500 text-sm mr-4">v{{ version }}</span>
        </div>
        <hr class="mt-2 border-zinc-700">
        <div class="flex flex-col">
            <div v-for="page in pages" :key="page.name">
                <router-link v-if="page.path" :to="page.path"
                    class="transition w-full flex text-sm items-center border-b border-transparent px-4 py-3.4 hover:border-accent">
                    <component :is="page.icon" class="text-white mb-0.5" />
                    <h1 class="text-white ml-2">{{ page.name }}</h1>
                </router-link>
                <hr v-else class="border-zinc-700 my-2">
            </div>
        </div>
    </div>
</template>
<style scoped>
.router-link-exact-active {
    border-color: rgba(222, 56, 71, var(--tw-bg-opacity));
}
</style>