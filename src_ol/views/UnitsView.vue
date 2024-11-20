<template>
    <div class="p-6 space-y-6">
      <h1 class="text-3xl font-semibold mb-4">Units Management</h1>
  
      <!-- Add New Unit Button -->
      <button class="btn btn-primary" @click="addUnit">Add New Unit</button>
  
      <!-- Units Table -->
      <sh-table :data="units" :columns="unitColumns" class="mt-4" />
  
      <!-- Edit Unit Modal (Optional) -->
      <sh-modal v-if="showModal" @close="closeModal">
        <h2 class="text-xl font-semibold mb-4">Edit Unit</h2>
        <form @submit.prevent="updateUnit">
          <sh-input v-model="selectedUnit.name" label="Unit Name" />
          <sh-select 
            v-model="selectedUnit.status" 
            label="Status" 
            :options="['Available', 'Busy', 'Off Duty']" 
          />
          <button type="submit" class="btn btn-primary mt-4">Save</button>
        </form>
      </sh-modal>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  
  interface Unit {
    id: number;
    name: string;
    status: string;
  }
  
  export default defineComponent({
    setup() {
      const units = ref<Unit[]>([]);
      const showModal = ref(false);
      const selectedUnit = ref<Unit | null>(null);
  
      const unitColumns = ref([
        { name: 'ID', key: 'id' },
        { name: 'Name', key: 'name' },
        { name: 'Status', key: 'status' },
      ]);
  
      const fetchUnits = async () => {
        try {
          const result = await invoke<Unit[]>('get_units');
          units.value = result;
        } catch (error) {
          console.error('Error fetching units:', error);
        }
      };
  
      const addUnit = () => {
        // Open modal or navigate to a form to add a new unit.
        console.log('Adding new unit...');
      };
  
      const updateUnit = () => {
        // Logic to update the selected unit's details.
        console.log('Unit updated:', selectedUnit.value);
        closeModal();
      };
  
      const closeModal = () => {
        showModal.value = false;
        selectedUnit.value = null;
      };
  
      fetchUnits();
  
      return { units, unitColumns, showModal, selectedUnit, addUnit, closeModal, updateUnit };
    },
  });
  </script>
  