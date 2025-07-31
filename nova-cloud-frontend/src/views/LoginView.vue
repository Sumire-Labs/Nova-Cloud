
<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'
import { animate, spring } from 'motion'

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

    // Save username to session storage to identify the user in other views
    sessionStorage.setItem('username', response.data.username);

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

const onButtonClick = (event: MouseEvent) => {
  const button = event.currentTarget as HTMLElement
  animate(button, 
    { scale: [1, 0.95, 1] }, 
    { duration: 0.3, easing: spring({ stiffness: 300, damping: 15}) }
  )
}
</script>

<template>
  <div class="w-full h-full flex items-center justify-center p-4">
    <div class="relative w-full max-w-md mx-auto p-8 bg-white/10 backdrop-blur-md rounded-2xl border border-white/20 shadow-2xl">
      <div class="text-center mb-8">
        <h1 class="text-4xl font-bold tracking-tighter text-white">Welcome Back</h1>
        <p class="mt-2 text-gray-300">Enter your credentials to access Nova Cloud.</p>
      </div>

      <form class="space-y-6" @submit.prevent="handleSubmit">
        <div>
          <label for="username" class="block mb-2 text-sm font-medium text-gray-200">Username</label>
          <input
            v-model="username"
            type="text"
            id="username"
            class="block w-full px-4 py-3 rounded-lg bg-white/10 border border-white/20 text-white placeholder-gray-400 focus:ring-2 focus:ring-violet-400 focus:border-violet-400 outline-none transition-all duration-300"
            placeholder="your-username"
            required
          />
        </div>

        <div>
          <label for="password" class="block mb-2 text-sm font-medium text-gray-200">Password</label>
          <input
            v-model="password"
            type="password"
            id="password"
            class="block w-full px-4 py-3 rounded-lg bg-white/10 border border-white/20 text-white placeholder-gray-400 focus:ring-2 focus:ring-violet-400 focus:border-violet-400 outline-none transition-all duration-300"
            placeholder="••••••••"
            required
          />
        </div>

        <div v-if="errorMessage" class="p-3 text-sm text-red-300 bg-red-500/20 rounded-lg" role="alert">
          {{ errorMessage }}
        </div>

        <button
          type="submit"
          @click="onButtonClick"
          class="w-full px-4 py-3 font-semibold text-white bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-4 focus:ring-violet-500/50 transition-all duration-300 transform active:scale-95"
        >
          Sign In
        </button>
      </form>

      <div class="text-center mt-6">
        <p class="text-sm text-gray-400">
          Don't have an account?
          <router-link to="/register" class="font-medium text-violet-400 hover:underline">Register Now</router-link>
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Scoped styles can be added here if needed, but most is handled by Tailwind */
</style>
