<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router'

const route = useRoute()
</script>

<template>
  <div class="app-container">
    <router-view v-slot="{ Component }">
      <transition :name="route.meta.transition || 'fade'" mode="out-in">
        <div :key="route.path" class="page">
          <component :is="Component" />
        </div>
      </transition>
    </router-view>
  </div>
</template>

<style>
.app-container {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

.page {
  position: absolute;
  width: 100%;
  height: 100%;
}

/* --- Fade Transition (Default) --- */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* --- Slide-Up Transition --- */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.5s ease;
}
.slide-up-enter-from {
  opacity: 0;
  transform: translateY(30px);
}
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(-30px);
}

/* --- Slide-Next/Prev Transition (for wizard-like flows) --- */
.slide-next-enter-active,
.slide-next-leave-active,
.slide-prev-enter-active,
.slide-prev-leave-active {
    transition: transform 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-next-enter-from {
    transform: translateX(100%);
}
.slide-next-leave-to {
    transform: translateX(-100%);
}

.slide-prev-enter-from {
    transform: translateX(-100%);
}
.slide-prev-leave-to {
    transform: translateX(100%);
}

</style>
