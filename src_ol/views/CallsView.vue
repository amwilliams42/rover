<template>
    <div class="p-6 space-y-6">
      <h1 class="text-3xl font-semibold mb-4">Calls for Service</h1>
  
      <!-- Filters -->
      <div class="flex gap-4 mb-4">
        <sh-select
          v-model="selectedPriority"
          label="Filter by Priority"
          :options="['All', 'High', 'Medium', 'Low']"
        />
        <sh-select
          v-model="selectedStatus"
          label="Filter by Status"
          :options="['All', 'Ongoing', 'Resolved']"
        />
      </div>
  
      <!-- Calls Table -->
      <sh-table :data="filteredCalls" :columns="callColumns" />
  
      <!-- Call Details Modal -->
      <sh-modal v-if="selectedCall" @close="closeCallDetails">
        <h2 class="text-xl font-semibold mb-4">Call Details</h2>
        <p><strong>Description:</strong> {{ selectedCall.description }}</p>
        <p><strong>Priority:</strong> {{ selectedCall.priority }}</p>
        <p><strong>Status:</strong> {{ selectedCall.status }}</p>
      </sh-modal>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, computed } from 'vue';
  import { invoke } from '@tauri-apps/api/core';

  
  interface Call {
    id: number;
    description: string;
    priority: string;
    status: string;
  }
  
  export default defineComponent({
    setup() {
      const calls = ref<Call[]>([]);
      const selectedPriority = ref('All');
      const selectedStatus = ref('All');
      const selectedCall = ref<Call | null>(null);
  
      const callColumns = ref([
        { name: 'ID', key: 'id' },
        { name: 'Description', key: 'description' },
        { name: 'Priority', key: 'priority' },
        { name: 'Status', key: 'status' },
      ]);
  
      const fetchCalls = async () => {
        try {
          const result = await invoke<Call[]>('get_calls');
          calls.value = result;
        } catch (error) {
          console.error('Error fetching calls:', error);
        }
      };
  
      const filteredCalls = computed(() =>
        calls.value.filter((call) => {
          const priorityMatch =
            selectedPriority.value === 'All' || call.priority === selectedPriority.value;
          const statusMatch =
            selectedStatus.value === 'All' || call.status === selectedStatus.value;
          return priorityMatch && statusMatch;
        })
      );
  
      const closeCallDetails = () => {
        selectedCall.value = null;
      };
  
      fetchCalls();
  
      return {
        calls,
        selectedPriority,
        selectedStatus,
        filteredCalls,
        callColumns,
        selectedCall,
        closeCallDetails,
      };
    },
  });
  </script>
  
  <style scoped>
  /* Optional: Custom CSS to enhance layout */
  </style>
  