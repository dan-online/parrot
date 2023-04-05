<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { ref } from 'vue';

import Package from "../components/Package.vue"

import { PopularInfo } from "../../src-tauri/bindings/PopularInfo"
import Spinner from '~icons/svg-spinners/180-ring-with-bg'

const packages = ref<PopularInfo[]>([]);
const loading = ref(true);

invoke("popular")

listen<PopularInfo[]>("popular", ({ payload }) => {
    packages.value = payload;
    loading.value = false;
})


</script>
<template>
    <div class="h-full">
        <Transition name="fade" mode="out-in">
            <div v-if="!loading" class="h-full overflow-y-auto grid">
                <Package v-for="pkg in packages" :key="pkg.name" :pkg="pkg"></Package>
            </div>
            <div v-else class="flex items-center justify-center h-2/3">
                <Spinner class="text-accent text-2xl"></Spinner>
            </div>
        </Transition>
    </div>
</template>