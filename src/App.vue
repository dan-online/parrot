<script setup lang="ts">
import SideBar from './components/SideBar.vue';
import Navbar from './components/Navbar.vue';
</script>
<template>
  <div class="bg-dark max-h-screen h-screen rounded flex border-zinc-700 overflow-hidden">
    <SideBar></SideBar>
    <div class="w-full">
      <Navbar></Navbar>
      <Suspense>
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </Suspense>
    </div>
  </div>
</template>

<style>
html,
body {
  background-color: transparent;
  font-family: 'Hind Madurai', sans-serif;
}

h1,
h2 {
  font-family: 'Montserrat', sans-serif;
}

h3,
h4,
h5,
h6 {
  font-family: 'Hind Madurai', sans-serif;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

