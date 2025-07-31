<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref, onMounted } from 'vue'

const router = useRouter()
// In a real app, you'd get the username from a store (Pinia) or a session
const username = ref('User') 

const handleLogout = () => {
  // Clear user session/token here in a real app
  console.log('Logging out...')
  router.push('/')
}

onMounted(() => {
  // You could fetch initial file data here
  console.log('MainView mounted.')
})
</script>

<template>
  <div class="flex flex-col h-screen bg-gray-50 dark:bg-gray-900">
    <!-- Header -->
    <header class="flex items-center justify-between p-4 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
      <h1 class="text-xl font-bold text-gray-900 dark:text-white">Nova Cloud</h1>
      <div class="flex items-center space-x-4">
        <span class="text-sm text-gray-600 dark:text-gray-300">Welcome, {{ username }}!</span>
        <button
          @click="handleLogout"
          class="px-4 py-2 text-sm font-medium text-white bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-4 focus:ring-violet-400 dark:focus:ring-violet-800 transition-all duration-200"
        >
          Logout
        </button>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 p-8 overflow-y-auto">
      <div class="text-center">
        <h2 class="text-2xl font-semibold text-gray-800 dark:text-gray-200">File Management</h2>
        <p class="mt-2 text-gray-500 dark:text-gray-400">Your files will appear here.</p>
        <!-- File list, upload button etc. will go here -->
      </div>
    </main>
  </div>
</template>

<style scoped>
/* Scoped styles for MainView if needed */
</style>
