<script setup lang="ts">
import { useRouter } from 'vue-router';


import type { PopularInfo } from '../../src-tauri/bindings/PopularInfo';
import type { BaseInfo } from '../../src-tauri/bindings/BaseInfo';
import type { AurInfo } from '../../src-tauri/bindings/AurInfo';

import Right from '~icons/material-symbols/chevron-right-rounded'
import Upvote from '~icons/bxs/upvote'
import Star from '~icons/material-symbols/star-rounded'
import Download from '~icons/fe/download'

export type pkg = PopularInfo | BaseInfo | AurInfo

const { pkg, extraClass } = defineProps<{ pkg: pkg, extraClass?: string }>();
const router = useRouter();

function go(pkg: pkg) {
    router.push(`/package/${pkg.repo}/${pkg.name}`)
}
</script>
<template>
    <div>
        <div @click="go(pkg)"
            class="flex hover:border-accent transition cursor-pointer border-b border-zinc-700 mb-2 py-4 px-2"
            :key="pkg.name">
            <div :class="['text-white', extraClass]">
                <div class="flex items-center">
                    <div class="flex items-baseline">
                        <h1>{{ pkg.repo }} / {{ pkg.name }}</h1>
                        <span class="text-sm ml-2">{{ pkg.version }}</span>
                    </div>
                    <div v-if="'installed' in pkg && pkg.installed"
                        class="text-xs font-medium text-black rounded px-2 ml-2 bg-accent">
                        Installed
                    </div>
                </div>
                <div class="text-sm">{{ pkg.description }}</div>

                <div v-if="'votes' in pkg && pkg.votes && 'popularity' in pkg && pkg.popularity"
                    class="text-sm flex items-center">
                    <Upvote class="text-zinc-400 mb-0.5 w-3 h-3"></Upvote>
                    <div class="ml-0.5 text-accent">{{ parseInt(pkg.votes.toString(), 10) }}
                    </div>
                    <div class="ml-0.5 text-zinc-400">votes</div>
                    <span class="text-zinc-400 ml-1">•</span>
                    <Star class="text-zinc-400 ml-1 mb-0.5 w-4 h-4"></Star>
                    <span class="text-accent ml-0.5">{{
                        pkg.popularity.startsWith("~") ? pkg.popularity.slice(1) : pkg.popularity
                    }}</span>
                    <span class="text-zinc-400 ml-1">popularity</span>
                </div>
                <div class="text-sm flex items-center" v-else-if="'download_size' in pkg && pkg.download_size">
                    <Download class="text-zinc-400 w-4 h-4"></Download>
                    <div class="ml-0.5 text-accent">{{ pkg.download_size }}</div>
                    <span class="text-zinc-400 ml-1">/</span>
                    <div class="ml-1 text-accent">{{ pkg.installed_size }}</div>
                </div>
            </div>
            <div class="ml-auto h-auto text-light flex items-center justify-center">
                <Right></Right>
            </div>
        </div>
    </div>
</template>