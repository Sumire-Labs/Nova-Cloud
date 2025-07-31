<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref, onMounted } from 'vue'
import { animate, spring } from 'motion'

const router = useRouter()
// In a real app, you'd get the username from a store (Pinia) or a session
const username = ref('User') 

const handleLogout = () => {
  console.log('Logging out...')
  router.push('/')
}

const onFabClick = (event: MouseEvent) => {
  const button = event.currentTarget as HTMLElement
  animate(button, 
    { scale: [1, 0.9, 1] }, 
    { duration: 0.4, easing: spring({ stiffness: 250, damping: 12}) }
  )
  // Placeholder for file upload logic
  alert('File upload initiated!')
}

onMounted(() => {
  console.log('MainView mounted.')
})
</script>

<template>
  <div class="relative w-full h-full flex flex-col p-4 sm:p-6 lg:p-8 gap-4">
    <!-- Header -->
    <header class="flex-shrink-0 flex items-center justify-between p-4 bg-white/10 backdrop-blur-md rounded-2xl border border-white/20 shadow-lg">
      <h1 class="text-xl font-bold text-white">Nova Cloud</h1>
      <div class="flex items-center space-x-4">
        <span class="text-sm text-gray-300 hidden sm:inline">Welcome, {{ username }}!</span>
        <button
          @click="handleLogout"
          class="px-4 py-2 text-sm font-medium text-white bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-4 focus:ring-violet-500/50 transition-all duration-300 transform active:scale-95"
        >
          Logout
        </button>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 p-6 bg-white/5 backdrop-blur-md rounded-2xl border border-white/10 shadow-lg overflow-y-auto">
      <div class="text-center text-white">
        <h2 class="text-2xl font-semibold">File Management</h2>
        <p class="mt-2 text-gray-400">Your files will appear here.</p>
        <!-- File list will be rendered here -->
      </div>
    </main>

    <!-- Floating Action Button -->
    <button 
      @click="onFabClick"
      class="absolute bottom-8 right-8 w-16 h-16 bg-violet-600 rounded-full shadow-2xl flex items-center justify-center text-white text-3xl font-light hover:bg-violet-500 focus:outline-none focus:ring-4 focus:ring-violet-500/50 transition-transform duration-200 ease-in-out hover:scale-110"
      aria-label="Upload file"
    >
      +
    </button>
  </div>
</template>

<style scoped>
/* Custom scrollbar for a more modern look */
main::-webkit-scrollbar {
  width: 8px;
}

main::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
}

main::-webkit-scrollbar-thumb {
  background: rgba(139, 92, 246, 0.6); /* Violet-500 with opacity */
  border-radius: 10px;
}

main::-webkit-scrollbar-thumb:hover {
  background: rgba(124, 58, 237, 0.8); /* Violet-600 with opacity */
}
</style>
