<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref, onMounted, computed } from 'vue'
import axios from 'axios'
import { animate, spring, stagger } from 'motion'

// --- Types ---
type DriveFile = {
  id: string;
  name: string;
  mimeType: string;
  size?: string;
  modifiedTime?: string;
  iconLink?: string;
};

// --- Refs and State ---
const router = useRouter()
const username = ref(sessionStorage.getItem('username') || 'User')
const files = ref<DriveFile[]>([])
const isLoading = ref(true)
const errorMessage = ref('')
const fileInputRef = ref<HTMLInputElement | null>(null)

// --- API Calls ---
const fetchFiles = async () => {
  if (!username.value) {
    errorMessage.value = 'Could not identify user.'
    isLoading.value = false
    return
  }
  try {
    isLoading.value = true
    const response = await axios.get(`http://localhost:3000/api/files/${username.value}`)
    files.value = response.data
    animateFiles()
  } catch (error: any) {
    console.error('Failed to fetch files:', error)
    errorMessage.value = 'Could not load files. Please try again later.'
  } finally {
    isLoading.value = false
  }
}

// --- UI Logic ---
const handleLogout = () => {
  sessionStorage.removeItem('username')
  router.push('/')
}

const triggerFileUpload = () => {
  fileInputRef.value?.click()
}

const handleFileSelected = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    const selectedFile = target.files[0]
    console.log('Selected file:', selectedFile.name)
    // TODO: Implement the actual upload logic
    alert(`You selected: ${selectedFile.name}`)
  }
}

const fileIcon = (mimeType: string) => {
  if (mimeType.includes('folder')) return '📁';
  if (mimeType.includes('image')) return '🖼️';
  if (mimeType.includes('pdf')) return '📄';
  if (mimeType.includes('video')) return '🎬';
  if (mimeType.includes('audio')) return '🎵';
  return '📝';
};

const formatSize = (size?: string) => {
  if (!size) return '--';
  const sizeInBytes = parseInt(size, 10);
  if (sizeInBytes < 1024) return `${sizeInBytes} B`;
  if (sizeInBytes < 1024 * 1024) return `${(sizeInBytes / 1024).toFixed(1)} KB`;
  return `${(sizeInBytes / (1024 * 1024)).toFixed(1)} MB`;
};

const animateFiles = () => {
  setTimeout(() => {
    animate(
      '.file-card',
      { opacity: [0, 1], transform: ['translateY(20px)', 'translateY(0)'] },
      { delay: stagger(0.05), duration: 0.5, easing: spring({ stiffness: 150, damping: 20 }) }
    )
  }, 100)
}

// --- Lifecycle Hooks ---
onMounted(() => {
  fetchFiles()
})
</script>

<template>
  <div class="relative w-full h-full">
    <!-- Main Layout with Padding -->
    <div class="flex flex-col w-full h-full p-4 sm:p-6 lg:p-8 gap-4">
      <!-- Header -->
      <header class="flex-shrink-0 flex items-center justify-between p-4 bg-white/10 backdrop-blur-md rounded-2xl border border-white/20 shadow-lg">
        <h1 class="text-xl font-bold text-white">Nova Cloud</h1>
        <div class="flex items-center space-x-4">
          <span class="text-sm text-gray-300 hidden sm:inline">Welcome, {{ username }}!</span>
          <button @click="handleLogout" class="px-4 py-2 text-sm font-medium text-white bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-4 focus:ring-violet-500/50 transition-all duration-300 transform active:scale-95">Logout</button>
        </div>
      </header>

      <!-- Main Content -->
      <main class="flex-1 p-2 sm:p-4 md:p-6 bg-white/5 backdrop-blur-md rounded-2xl border border-white/10 shadow-lg overflow-y-auto">
        <div v-if="isLoading" class="flex justify-center items-center h-full">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-violet-400"></div>
        </div>
        <div v-else-if="errorMessage" class="text-center text-red-300 p-8">
          <p>{{ errorMessage }}</p>
        </div>
        <div v-else-if="files.length === 0" class="text-center text-gray-400 p-8">
          <p>No files found. Upload your first file!</p>
        </div>
        <div v-else class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
          <div v-for="file in files" :key="file.id" class="file-card opacity-0 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg p-4 flex flex-col items-center justify-center text-center cursor-pointer transition-all duration-200 transform hover:-translate-y-1">
            <div class="text-4xl mb-2">{{ fileIcon(file.mimeType) }}</div>
            <p class="text-sm text-white font-medium truncate w-full" :title="file.name">{{ file.name }}</p>
            <p class="text-xs text-gray-400">{{ formatSize(file.size) }}</p>
          </div>
        </div>
      </main>
    </div>

    <!-- Hidden File Input -->
    <input type="file" ref="fileInputRef" @change="handleFileSelected" class="hidden" />

    <!-- Floating Action Button -->
    <button @click="triggerFileUpload" class="absolute bottom-8 right-8 w-16 h-16 bg-violet-600 rounded-full shadow-2xl flex items-center justify-center text-white text-3xl font-light hover:bg-violet-500 focus:outline-none focus:ring-4 focus:ring-violet-500/50 transition-transform duration-200 ease-in-out hover:scale-110" aria-label="Upload file">+</button>
  </div>
</template>

<style scoped>
main::-webkit-scrollbar { width: 8px; }
main::-webkit-scrollbar-track { background: rgba(255, 255, 255, 0.05); border-radius: 10px; }
main::-webkit-scrollbar-thumb { background: rgba(139, 92, 246, 0.6); border-radius: 10px; }
main::-webkit-scrollbar-thumb:hover { background: rgba(124, 58, 237, 0.8); }
</style>