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
  background-color: #f0f2f5; /* Consistent background */
}

.page {
  position: absolute;
  width: 100%;
  height: 100%;
}

/* --- Shared Axis (Z-axis) Transition --- */
.shared-axis-forward-enter-active,
.shared-axis-forward-leave-active,
.shared-axis-backward-enter-active,
.shared-axis-backward-leave-active {
  transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

/* Forward (e.g., Login -> Register) */
.shared-axis-forward-enter-from {
  transform: translateX(30px);
  opacity: 0;
}
.shared-axis-forward-leave-to {
  transform: translateX(-30px);
  opacity: 0;
}

/* Backward (e.g., Register -> Login) */
.shared-axis-backward-enter-from {
  transform: translateX(-30px);
  opacity: 0;
}
.shared-axis-backward-leave-to {
  transform: translateX(30px);
  opacity: 0;
}

</style>
