<template>
  <div id="app">
    <h1>File Indexer</h1>
    <button @click="indexFiles" :disabled="loading">Index Files</button>
    <input @keydown.enter="searchFile" v-model="fileName" placeholder="Enter file name" />
    <input v-model="searchQuery" placeholder="Search files" />
    <p>{{ result }}</p>

    <div v-if="loading" class="spinner"></div>

    <table v-if="!loading">
      <thead>
        <tr>
          <th>File Name</th>
          <th>Size (bytes)</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="file in filteredFiles" :key="file.name">
          <td><a href="#" @click.prevent="openFile(file.name)">{{ file.name }}</a></td>
          <td>{{ file.size }}</td>
        </tr>
      </tbody>
    </table>
    <div v-if="!loading && !searchQuery">
      <button @click="loadMore" :disabled="loading">Load More</button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api'

const fileName = ref('')
const result = ref('')
const indexedFiles = ref([])
const displayedFiles = ref([])
const loading = ref(false)
const searchQuery = ref('')
const offset = ref(0)
const limit = ref(100)

const indexFiles = async () => {
  loading.value = true
  await invoke('index_files')
  alert('Indexing completed')
  offset.value = 0
  indexedFiles.value = []
  displayedFiles.value = []
  loadAllFiles()
  loading.value = false
}

const searchFile = async () => {
  const response = await invoke('search_file', { fileName: fileName.value })
  result.value = response ? `File found: ${response}` : 'File not found'
}

const loadAllFiles = async () => {
  loading.value = true
  let allFiles = []
  let tempOffset = 0
  while (true) {
    const files = await invoke('get_indexed_files', { offset: tempOffset, limit: limit.value })
    if (files.length === 0) break
    allFiles = allFiles.concat(files.map(file => ({
      name: file[0],
      size: file[1],
      path: file[2]
    })))
    tempOffset += limit.value
  }
  indexedFiles.value = allFiles
  displayedFiles.value = allFiles.slice(0, limit.value)
  offset.value = limit.value
  loading.value = false
}

const loadMore = () => {
  const nextBatch = indexedFiles.value.slice(offset.value, offset.value + limit.value)
  displayedFiles.value = displayedFiles.value.concat(nextBatch)
  offset.value += limit.value
}

const openFile = async (fileName) => {
  const filePath = await invoke('search_file', { fileName })
  console.log('Opening file:', filePath) // Debug log to check filePath
  if (filePath) {
    await invoke('open_file_in_explorer', { filePath: filePath })
  } else {
    console.error('File path not found')
  }
}

const filteredFiles = computed(() => {
  if (searchQuery.value) {
    return indexedFiles.value.filter(file => file.name.toLowerCase().includes(searchQuery.value.toLowerCase()))
  } else {
    return displayedFiles.value
  }
})

onMounted(loadAllFiles)
</script>

<style>
#app {
  text-align: center;
}

table {
  margin-top: 20px;
  width: 100%;
  border-collapse: collapse;
}

th, td {
  border: 1px solid #ddd;
  padding: 8px;
}

th {
  background-color: #f2f2f2;
  text-align: left;
}

a {
  color: blue;
  cursor: pointer;
  text-decoration: underline;
}

.spinner {
  border: 16px solid #f3f3f3;
  border-top: 16px solid #3498db;
  border-radius: 50%;
  width: 120px;
  height: 120px;
  animation: spin 2s linear infinite;
  margin: 20px auto;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>
