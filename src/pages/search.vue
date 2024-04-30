<script setup lang="ts">
import Fuse, { FuseResult } from "fuse.js";
import { nextTick, onBeforeUnmount, ref } from "vue";
import { useFuse } from '@vueuse/integrations/useFuse'
import { useRoute, useRouter } from "vue-router";

import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'

import { SearchInfo } from "../../src-tauri/bindings/SearchInfo";

import Spinner from '~icons/svg-spinners/180-ring-with-bg'
import Right from '~icons/material-symbols/chevron-right-rounded'
import Upvote from '~icons/bxs/upvote'
import Star from '~icons/material-symbols/star-rounded'
import Download from '~icons/fe/download'

const router = useRouter()
const route = useRoute();
const query = ref(route.query.q as string || "");
const local = ref(false);
const loading = ref(false);

let results = ref<FuseResult<SearchInfo>[]>([]);

async function search() {
    loading.value = true;
    invoke<SearchInfo[]>("search", { query: query.value, local: local.value });
}

async function go(item: SearchInfo) {
    router.push(`/package/${item.repo}/${item.name}`)
}

const listener = await listen<SearchInfo[]>("search", ({ payload }) => {
    const { results: res } = useFuse(query, payload, { fuseOptions: { keys: ['name', 'repo'], threshold: 1 } })
    results.value = res.value;
    nextTick(() => {
        loading.value = false;
    })
});

onBeforeUnmount(() => {
    listener();
});

if (query.value) {
    search();
}

</script>

<template>
    <div class="h-full px-4 w-full">
        <div class="flex space-x-4 w-full pt-4">
            <input @keyup="$event => $event.key === 'Enter' && search()"
                class="w-full bg-transparent rounded-sm border-2 border-accent px-4 text-light focus:text-white transition outline-none ring-0 py-2"
                v-model="query" placeholder="Enter a package..." />
            <button class="px-6 py-2 rounded bg-accent text-gray-200" type="button" @click="search()">Search</button>
        </div>
        <Transition name="fade" mode="out-in">
            <div v-if="!loading" class="flex flex-col overflow-auto h-6/8 pr-4 mt-4">
                <div v-for="result, idx in results" @click="go(result.item)"
                    :class="['flex hover:border-accent transition cursor-pointer border-b mb-2', idx > 0 ? 'pt-2 pb-4' : 'pb-4']"
                    :key="result.item.name">
                    <div class="text-white">
                        <div class="flex items-center">
                            <div class="flex items-baseline">
                                <h1>{{ result.item.repo }} / {{ result.item.name }}</h1>
                                <span class="text-sm ml-2">{{ result.item.version }}</span>
                            </div>
                            <div v-if="result.item.installed"
                                class="text-xs font-medium text-black rounded px-2 ml-2 bg-accent">
                                Installed
                            </div>
                        </div>
                        <div class="text-sm">{{ result.item.description }}</div>

                        <div v-if="result.item.votes && result.item.popularity" class="text-sm flex items-center">
                            <Upvote class="text-zinc-400 mb-0.5 w-3 h-3"></Upvote>
                            <div class="ml-0.5 text-accent">{{ parseInt(result.item.votes, 10) }}
                            </div>
                            <div class="ml-0.5 text-zinc-400">votes</div>
                            <span class="text-zinc-400 ml-1">•</span>
                            <Star class="text-zinc-400 ml-1 mb-0.5 w-4 h-4"></Star>
                            <span class="text-accent ml-0.5">{{
                                result.item.popularity.slice(1)
                            }}</span>
                            <span class="text-zinc-400 ml-1">popularity</span>
                        </div>
                        <div class="text-sm flex items-center" v-else>
                            <Download class="text-zinc-400 w-4 h-4"></Download>
                            <div class="ml-0.5 text-accent">{{ result.item.download_size }}</div>
                            <span class="text-zinc-400 ml-1">/</span>
                            <div class="ml-1 text-accent">{{ result.item.installed_size }}</div>
                        </div>
                    </div>
                    <div class="ml-auto h-auto text-light flex items-center justify-center">
                        <Right></Right>
                    </div>
                </div>
            </div>
            <div v-else class="flex items-center justify-center h-2/3">
                <Spinner class="text-accent text-2xl"></Spinner>
            </div>
        </Transition>
    </div>
</template>

