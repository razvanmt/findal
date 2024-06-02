<template>
  <div id="app">
    <h1>findal</h1>
    <button @click="indexFiles">Index Files</button>
    <input v-model="fileName" placeholder="Enter file name" />
    <button @click="searchFile">Search</button>
    <p>{{ result }}</p>
    <table>
      <thead>
        <tr>
          <th>File Name</th>
          <th>Size (bytes)</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="file in indexedFiles" :key="file.name">
          <td><a href="#" @click.prevent="openFile(file.name)">{{ file.name }}</a></td>
          <td>{{ file.size }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'

const fileName = ref('')
const result = ref('')
const indexedFiles = ref([])

const indexFiles = async () => {
  await invoke('index_files')
  alert('Indexing completed')
  fetchIndexedFiles()
}

const searchFile = async () => {
  const response = await invoke('search_file', { fileName: fileName.value })
  result.value = response ? `File found: ${response}` : 'File not found'
}

const fetchIndexedFiles = async () => {
  const files = await invoke('get_indexed_files')
  indexedFiles.value = files.map(file => ({
    name: file[0],
    size: file[1]
  }))
  console.log('Indexed Files:', indexedFiles.value) // Add this line to debug
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

onMounted(fetchIndexedFiles)
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
</style>
