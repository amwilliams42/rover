<template>
    <aside :class="['w-64', { 'hidden': !isPanelVisible }]" class="bg-gray-200 p-4 border-l border-gray-300">
      <button @click="togglePanel" class="btn btn-secondary mb-4">
        {{ isPanelVisible ? 'Hide' : 'Show' }} Panel
      </button>
      <ToolsPanel />
      <FiltersPanel />
      <UnitStatusPanel />
    </aside>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  import ToolsPanel from './Palettes/ToolsPanel.vue';
  import FiltersPanel from './Palettes/FiltersPanel.vue';
  import UnitStatusPanel from './Palettes/UnitStatusPanel.vue';
  
  export default defineComponent({
    components: { ToolsPanel, FiltersPanel, UnitStatusPanel },
    setup() {
      const isPanelVisible = ref(true);
  
      function togglePanel() {
        isPanelVisible.value = !isPanelVisible.value;
      }
  
      return { isPanelVisible, togglePanel };
    },
  });
  </script>
  