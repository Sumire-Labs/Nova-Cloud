<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router'

const route = useRoute()
</script>

<template>
  <div class="relative w-full h-screen overflow-hidden bg-slate-900">
    <div class="absolute inset-0 bg-[radial-gradient(ellipse_80%_80%_at_50%_-20%,rgba(120,119,198,0.3),rgba(255,255,255,0))] animate-aurora"></div>
    
    <router-view v-slot="{ Component }">
      <transition :name="route.meta.transition || 'fade'" mode="out-in">
        <div :key="route.path" class="w-full h-full">
          <component :is="Component" />
        </div>
      </transition>
    </router-view>
  </div>
</template>

<style>
/* Using a more specific selector to override default body styles if necessary */
body, #app {
  margin: 0;
  padding: 0;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* Shared Axis Transition (unchanged) */
.shared-axis-forward-enter-active,
.shared-axis-forward-leave-active,
.shared-axis-backward-enter-active,
.shared-axis-backward-leave-active {
  transition: transform 0.4s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}
.shared-axis-forward-enter-from {
  transform: translateX(30px);
  opacity: 0;
}
.shared-axis-forward-leave-to {
  transform: translateX(-30px);
  opacity: 0;
}
.shared-axis-backward-enter-from {
  transform: translateX(-30px);
  opacity: 0;
}
.shared-axis-backward-leave-to {
  transform: translateX(30px);
  opacity: 0;
}

/* Default fade for other transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
