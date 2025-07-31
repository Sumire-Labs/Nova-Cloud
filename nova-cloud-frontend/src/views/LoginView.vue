
<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

const username = ref('')
const password = ref('')
const errorMessage = ref('')
const router = useRouter()

const handleSubmit = async () => {
  errorMessage.value = ''
  try {
    const response = await axios.post('http://localhost:3000/login', {
      username: username.value,
      password: password.value,
    })
    console.log('Backend response:', response.data)
    // On successful login, navigate to the main view
    router.push('/main') 
  } catch (error: any) {
    console.error('Error during login:', error)
    if (error.response) {
      errorMessage.value = error.response.data.message || 'Invalid username or password.'
    } else {
      errorMessage.value = 'An unexpected error occurred. Please try again.'
    }
  }
}
</script>

<template>
  <main class="flex items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900 font-sans">
    <div class="w-full max-w-md p-10 space-y-6 bg-white dark:bg-gray-800 rounded-2xl shadow-xl">
      <div class="text-center">
        <h1 class="text-4xl font-bold tracking-tighter text-gray-900 dark:text-white">Welcome Back!</h1>
        <p class="mt-2 text-gray-600 dark:text-gray-400">Sign in to access your cloud.</p>
      </div>

      <form class="space-y-6" @submit.prevent="handleSubmit">
        <div>
          <label for="username" class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">Username</label>
          <input
            v-model="username"
            type="text"
            id="username"
            class="block w-full px-4 py-3 border rounded-lg bg-gray-50 dark:bg-gray-700 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white focus:ring-violet-500 focus:border-violet-500 transition-all duration-300"
            placeholder="your-username"
            required
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
            required
          />
        </div>

        <div v-if="errorMessage" class="p-3 text-sm text-red-700 bg-red-100 rounded-lg dark:bg-red-200 dark:text-red-800" role="alert">
          {{ errorMessage }}
        </div>

        <button
          type="submit"
          class="w-full px-4 py-3 font-semibold text-white bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-4 focus:ring-violet-400 dark:focus:ring-violet-800 transition-all duration-200 active:scale-95"
        >
          Sign In
        </button>
      </form>

      <div class="text-center">
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Don't have an account?
          <router-link to="/register" class="font-medium text-violet-500 hover:underline">Register Now</router-link>
        </p>
      </div>
    </div>
  </main>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;900&display=swap');

.font-sans {
  font-family: 'Inter', sans-serif;
}

.shadow-xl {
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.rounded-2xl {
  border-radius: 1rem;
}
</style>
