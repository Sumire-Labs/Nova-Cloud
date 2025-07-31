
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { animate, spring } from 'motion'
import axios from 'axios'

const formEl = ref<HTMLElement | null>(null)
const username = ref('')
const password = ref('')

const handleLogin = async () => {
  try {
    const response = await axios.post('http://localhost:3000/login', {
      username: username.value,
      password: password.value,
    })
    console.log('Backend response:', response.data)
    alert('Login successful!') // Placeholder for success feedback
  } catch (error) {
    console.error('Error during login:', error)
    alert('Login failed!') // Placeholder for error feedback
  }
}

onMounted(() => {
  if (formEl.value) {
    animate(
      formEl.value,
      { y: [20, 0], opacity: [0, 1] },
      {
        delay: 0.2,
        duration: 0.8,
        easing: spring({ stiffness: 100, damping: 15 }),
      }
    )
  }
})
</script>

<template>
  <main class="flex items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900 font-sans">
    <div ref="formEl" class="w-full max-w-md p-10 space-y-8 bg-white dark:bg-gray-800 rounded-2xl shadow-xl">
      <div class="text-center">
        <h1 class="text-4xl font-bold tracking-tighter text-gray-900 dark:text-white">Nova Cloud</h1>
        <p class="mt-2 text-gray-600 dark:text-gray-400">Welcome back. Please sign in.</p>
      </div>

      <form class="space-y-6" @submit.prevent="handleLogin">
        <div>
          <label for="username" class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">Username</label>
          <input
            v-model="username"
            type="text"
            id="username"
            class="block w-full px-4 py-3 border rounded-lg bg-gray-50 dark:bg-gray-700 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white focus:ring-violet-500 focus:border-violet-500 transition-all duration-300"
            placeholder="your-username"
          />
        </div>

        <div>
          <label for="password" class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">Password</label>
          <input
            v-model="password"
            type="password"
            id="password"
            class="block w-full px-4 py-3 border rounded-lg bg-gray-50 dark:bg-gray-700 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white focus:ring-violet-500 focus:border-violet-500 transition-all duration-300"
            placeholder="••••••••"
          />
        </div>

        <button
          type="submit"
          class="w-full px-4 py-3 font-semibold text-white bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-4 focus:ring-violet-400 dark:focus:ring-violet-800 transition-all duration-200 active:scale-95"
        >
          Sign In
        </button>
      </form>
    </div>
  </main>
</template>
