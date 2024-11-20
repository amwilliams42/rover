<template>
    <main class="flex-1 bg-white p-4">
      <h2 class="text-2xl font-semibold mb-4">Dispatch Canvas</h2>
      <router-view /> <!-- Loads different views -->
    </main>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({});
  </script>
  