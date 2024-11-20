<template>
    <div class="p-6 space-y-6">
      <h1 class="text-3xl font-semibold mb-4">Dispatch Dashboard</h1>
  
      <!-- Quick Stats Section -->
      <div class="grid grid-cols-3 gap-6">
        <sh-card class="p-4">
          <h2 class="text-lg font-semibold">Active Units</h2>
          <p class="text-4xl font-bold mt-2">{{ activeUnits }}</p>
        </sh-card>
  
        <sh-card class="p-4">
          <h2 class="text-lg font-semibold">Ongoing Calls</h2>
          <p class="text-4xl font-bold mt-2">{{ ongoingCalls.length }}</p>
        </sh-card>
  
        <sh-card class="p-4">
          <h2 class="text-lg font-semibold">Available Units</h2>
          <p class="text-4xl font-bold mt-2">{{ availableUnits }}</p>
        </sh-card>
      </div>
  
      <!-- Active Calls List -->
      <div>
        <h2 class="text-xl font-semibold mb-4">Ongoing Calls</h2>
        <sh-table :data="ongoingCalls" :columns="callColumns" />
      </div>
  
      <!-- Quick Map Section (Placeholder) -->
      <div class="mt-6">
        <h2 class="text-xl font-semibold mb-4">Unit Locations</h2>
        <div class="w-full h-96 bg-gray-300 flex items-center justify-center">
          <p class="text-gray-700">Map Placeholder</p>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, onMounted } from 'vue';
  
  interface Call {
    id: number;
    description: string;
    priority: string;
  }
  
  export default defineComponent({
    setup() {
      // State variables with type annotations
      const activeUnits = ref<number>(5);
      const availableUnits = ref<number>(3);
      const ongoingCalls = ref<Call[]>([]);
  
      // Columns for sh-table component
      const callColumns = ref([
        { name: 'ID', key: 'id' },
        { name: 'Description', key: 'description' },
        { name: 'Priority', key: 'priority' },
      ]);
  
      // Fetch data from backend (example logic)
      const fetchData = async () => {
        try {
          // Replace with actual Tauri backend calls
          ongoingCalls.value = [
            { id: 1, description: 'Medical Emergency', priority: 'High' },
            { id: 2, description: 'Lost Child', priority: 'Medium' },
          ];
        } catch (error) {
          console.error('Failed to fetch data:', error);
        }
      };
  
      // Load data when the component mounts
      onMounted(fetchData);
  
      return {
        activeUnits,
        availableUnits,
        ongoingCalls,
        callColumns,
      };
    },
  });
  </script>
  
  <style scoped>
  /* Optional: Add any scoped CSS to enhance styling */
  </style>
  