<script setup lang="ts">
import { RecycleScroller } from 'vue-virtual-scroller'
import { computed, ref, watch } from 'vue'

import { SearchInfo } from '../../src-tauri/bindings/SearchInfo';
import { BaseInfo } from '../../src-tauri/bindings/BaseInfo'

import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

import Package from "../components/Package.vue"

import Spinner from '~icons/svg-spinners/180-ring-with-bg'

import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

import type { pkg } from "../components/Package.vue"


const loading = ref(true);
const packages = ref<BaseInfo[]>([])
const searchPackages = ref<SearchInfo[]>([])
const query = ref("")

listen<BaseInfo[]>('installed', ({ payload }) => {
    packages.value = payload
    loading.value = false
});

invoke('installed');

const pkgs = computed(() => {
    // @ts-expect-error 2339 Rust returns it weirdly
    return packages.value.map(x => x.Base as BaseInfo)
})

const search = async () => {
    if (query.value === "") {
        invoke('installed')
        return
    }

    const id = crypto.randomUUID();
    invoke('installed_search', { query: query.value, id })

    const stop = await listen<SearchInfo[]>('installed_search_' + id, ({ payload }) => {
        searchPackages.value = payload
        stop();
    })
}

const results = computed(() => {
    return query.value.length > 0 ? searchPackages.value : pkgs.value
})

watch(query, search)

</script>
<template>
    <div class="h-full">
        <div class="flex px-4 space-x-4 w-full pt-4">
            <input
                class="w-full bg-transparent rounded-sm border-2 border-accent px-4 text-light focus:text-white transition outline-none ring-0 py-2"
                v-model="query" placeholder="Enter a package..." />
        </div>
        <Transition name="fade" mode="out-in">
            <div v-if="!loading">
                <RecycleScroller class="scroller h-[30.6rem]" :item-size="77" :items="results" key-field="name"
                    v-slot="{ item }: { item: pkg }">
                    <Package class="px-4" :pkg="item"></Package>
                </RecycleScroller>
            </div>
            <div v-else class="flex items-center justify-center h-2/3">
                <Spinner class="text-accent text-2xl"></Spinner>
            </div>
        </Transition>
    </div>
</template>