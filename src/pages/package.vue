<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import { FullInfo } from "../../src-tauri/bindings/FullInfo"
import type { AurInfo } from "../../src-tauri/bindings/AurInfo"
import { Log } from "../../src-tauri/bindings/Log"
import { BaseInfo } from "../../src-tauri/bindings/BaseInfo"
import { useRoute } from "vue-router";

import { open } from '@tauri-apps/api/shell';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

import Spinner from '~icons/svg-spinners/180-ring-with-bg'
import Upvote from '~icons/bxs/upvote'
import Star from '~icons/material-symbols/star-rounded'
import Download from '~icons/fe/download'

const rawPkg = ref<FullInfo[]>();
const logs = ref<Log[]>([]);
const loading = ref(true);
const loadingInstruction = ref(false);
const logsRef = ref<HTMLElement | null>(null);
const logsStatus = ref(0) // 0 = not shown, 1 = shown, 2 = shown and done
const conflicts = ref(false);
const optionalDeps = ref(false);
const chosenOptionals = ref<string[]>([]);
const route = useRoute();

const id = crypto.randomUUID();

listen<FullInfo[]>("get-" + id, ({ payload }) => {
    console.log(payload)
    rawPkg.value = payload;
    loading.value = false;
})

listen<Log>("log", ({ payload }) => {
    logs.value.push(payload);

    if (logsStatus.value == 0) {
        logsStatus.value = 1;
    }
    if (payload.done) {
        logsStatus.value = 2;
        update();
    }
    if (logsRef.value) {
        nextTick(() => {
            logsRef.value!.scrollTop = logsRef.value!.scrollHeight;
        })
    }
})

const { repo, name } = route.params;

const update = () => {
    invoke("get", { name, repo, id })
    loadingInstruction.value = false;
}

const close = () => {
    logsStatus.value = 0;
    logs.value = [];
}

update();

const pkgs = computed(() => {
    if (repo === 'aur') {
        // @ts-expect-error 2339 Rust formats the JSON differently
        return rawPkg.value?.map(x => x && x.Aur as AurInfo) || [];
    } else {
        // @ts-expect-error 2339 Rust formats the JSON differently
        return rawPkg.value?.map(x => x && x.Base as BaseInfo) || [];
    }
})

const localPkg = computed(() => {
    return pkgs.value[0];
})

const remotePkg = computed(() => {
    return pkgs.value[1];
})

function run(instruction: string, skippedOptionals = false) {
    console.log(instruction)
    if (instruction === "install") {
        if (remotePkg.value.conflicts_with.length > 0) {
            conflicts.value = true;
            return;
        }

        if (remotePkg.value.optional_deps.length > 0 && !skippedOptionals) {
            optionalDeps.value = true;
            return;
        }
    }

    loadingInstruction.value = true;
    optionalDeps.value = false;

    const command = { instruction, package: remotePkg.value ? remotePkg.value.name : localPkg.value.name, optionalDeps: Object.values(chosenOptionals.value) };

    invoke("run_instruction", command)
}

const pkg = remotePkg;

const isInstalled = computed(() => {
    return localPkg.value !== null;
})

const isUpgradable = computed(() => {
    return localPkg.value && remotePkg.value && localPkg.value.version !== remotePkg.value.version;
})


</script>

<template>
    <div class="h-full text-light">
        <Transition name="fade" mode="out-in">
            <div v-if="logsStatus > 0"
                class="fixed flex items-center justify-center bg-[rgba(0,0,0,0.8)] top-0 left-0 h-full w-full">
                <div class="bg-zinc-800 rounded p-2 w-2/3 text-sm">
                    <h1 v-if="logsStatus == 1" class="text-2xl text-gray-200">Please wait...</h1>
                    <h1 v-else class="text-2xl text-gray-200">Done!</h1>
                    <hr class="border-zinc-700 mt-2 mb-4">
                    <div class="overflow-y-scroll max-h-64 bg-zinc-900 rounded p-2" ref="logsRef">
                        <code
                            class="rounded bg-zinc-900"><p v-for="log in logs" :key="log.line">[<span class="bg-green-500" v-if="log.stdout">O</span><span class="bg-red-500" v-else>E</span>] {{ log.line }}</p></code>
                    </div>
                    <button @click="close"
                        class="px-6 py-2 rounded bg-accent disabled:opacity-50 mt-3 disabled:bg-zinc-900 text-gray-200 w-32 flex items-center justify-center"
                        :disabled="logsStatus != 2">
                        Continue
                    </button>

                </div>
            </div>

        </Transition>
        <Transition name="fade" mode="out-in">
            <div v-if="conflicts"
                class="fixed flex items-center justify-center bg-[rgba(0,0,0,0.8)] top-0 left-0 h-full w-full">
                <div class="bg-zinc-800 rounded p-2 w-2/3 text-sm">
                    <h1 class="text-2xl text-gray-200">Conflict</h1>
                    <hr class="border-zinc-700 my-2">
                    <p>
                        {{ pkg.name }} conflicts with: <router-link v-for="c in pkg.conflicts_with"
                            class="text-accent underline mr-2" :to="`/search?q=${c}`">
                            {{ c }}</router-link>
                    </p>
                    <button @click="conflicts = false"
                        class="px-6 py-2 rounded bg-zinc-700 disabled:opacity-50 mt-3 disabled:bg-zinc-900 text-gray-200 w-32 flex items-center justify-center">
                        Dismiss
                    </button>
                </div>
            </div>
        </Transition>
        <Transition name="fade" mode="out-in">
            <div v-if="optionalDeps"
                class="fixed flex items-center justify-center bg-[rgba(0,0,0,0.8)] top-0 left-0 h-full w-full">
                <div class="bg-zinc-800 rounded p-2 w-2/3 text-sm">
                    <h1 class="text-2xl text-gray-200">Optional</h1>
                    <hr class="border-zinc-700 my-2">
                    <div v-if="!Array.isArray(pkg.optional_deps[0])">
                        <div class="flex items-center" v-for="optional in (pkg.optional_deps as string[])">
                            <input type="checkbox" class="w-4 accent-accent h-4" :id="optional" :value="optional"
                                v-model="chosenOptionals">
                            <label class="ml-2 text-xl" :for="optional">{{ optional }}</label>
                        </div>
                    </div>
                    <div v-else>
                        <div class="flex items-center" v-for="optional in (pkg.optional_deps as string[][])">
                            <input type="checkbox" class="w-4 accent-accent h-4" :id="optional[0]" :value="optional[0]"
                                v-model="chosenOptionals">
                            <label class="ml-2 items-center text-xl" :for="optional[0]">{{ optional[0] }} ({{ optional[1]
                            }})</label>
                        </div>
                    </div>
                    <div class="flex">
                        <button @click="optionalDeps = false"
                            class="px-6 py-2 rounded bg-zinc-700 disabled:opacity-50 mt-3 disabled:bg-zinc-900 text-gray-200 w-32 flex items-center justify-center">
                            Cancel
                        </button>
                        <button @click="run('install', true)"
                            class="px-6 py-2 ml-auto rounded bg-accent disabled:opacity-50 mt-3 disabled:bg-zinc-900 text-gray-200 w-32 flex items-center justify-center">
                            Continue
                        </button>
                    </div>
                </div>
            </div>
        </Transition>
        <Transition name="fade" mode="out-in">
            <div class="h-full" v-if="!loading">
                <div class="flex items-center px-4 py-2">
                    <h1 class="text-2xl">{{ pkg.repo }} / {{ pkg.name }}</h1>
                    <div class="ml-4 mt-0.5">
                        <!-- if aur repo then show votes + popularity -->
                        <div v-if="pkg.repo === 'aur'" class="flex items-center space-x-2">
                            <Upvote class="w-4 h-4 mb-0.5"></Upvote>
                            <p class="text-accent text-lg">{{ (pkg as AurInfo).votes }}</p>
                            <span class="text-gray-400">•</span>
                            <Star class="w-5 h-5 mb-1"></Star>
                            <p class="text-accent text-lg">{{ parseFloat((pkg as AurInfo).popularity).toFixed(2) }}</p>
                        </div>
                        <!-- if base repo then show size -->
                        <div v-else class="flex items-center space-x-2">
                            <Download class="w-4 h-4 mb-0.5"></Download>
                            <p class="text-accent text-lg">{{ (pkg as BaseInfo).installed_size }}</p>
                        </div>

                    </div>
                    <div class="ml-auto">
                        <button
                            class="px-6 py-2 rounded bg-zinc-900 text-gray-200 w-32 h-10 flex items-center justify-center"
                            v-if="loadingInstruction">
                            <Spinner></Spinner>
                        </button>
                        <button @click="run('install')" v-else-if="!isInstalled"
                            class="px-6 py-2 rounded bg-accent text-gray-200">Install</button>
                        <button @click="run('upgrade')" v-else-if="isUpgradable"
                            class="px-6 py-2 rounded bg-accent text-gray-200">Update
                            {{ remotePkg.version }}</button>
                        <button @click="run('remove')" v-else
                            class="px-6 py-2 rounded bg-accent text-gray-200">Uninstall</button>
                    </div>
                </div>
                <hr class="border-zinc-700">
                <div class="mt-2 pr-2 overflow-auto w-full h-[80%]">
                    <table class="w-full">
                        <tbody>
                            <tr v-for="[key, val] in Object.entries(pkg).filter(x => x[1] && x[1].length > 0 && x[1] != 'None' && !(Array.isArray(x[1]) && x[1][0] == 'None'))"
                                class="border-b border-zinc-700">
                                <td class="px-4 py-2">{{ key }}</td>
                                <td class="px-4 py-2">
                                    <div v-if="Array.isArray(val) && Array.isArray(val[0])">
                                        <p v-for="x in val">
                                            <router-link :to="'/search?q=' + x[0]" class="text-accent underline">{{ x[0]
                                            }}</router-link>: {{ x[1] }}
                                        </p>
                                    </div>
                                    <span v-else-if="Array.isArray(val)">
                                        <p v-for="v in val">
                                            <router-link v-if="key.toLowerCase().includes('dep')" :to="'/search?q=' + v"
                                                class="text-accent underline">{{ v
                                                }}</router-link>
                                            <span v-else>{{ v }}</span>
                                        </p>
                                    </span>
                                    <span v-else-if="typeof val === 'string' && val.startsWith('http')">
                                        <button @click="open(val)" class="text-accent underline">{{ val }}</button>
                                    </span>
                                    <span v-else>{{ val }}</span>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
            <div v-else class="flex items-center justify-center h-2/3">
                <Spinner class="text-accent text-2xl"></Spinner>
            </div>
        </Transition>
    </div>
</template>