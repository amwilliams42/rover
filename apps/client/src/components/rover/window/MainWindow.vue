<template>
    <div class="flex flex-col h-screen bg-[#1e1e1e] text-[#cccccc]">
      <MenuBar data-tauri-drag-region />
      <div class="h-screen flex">
    <!-- Left Sidebar with Compact Tool Icons -->
    <aside class="sidebar flex flex-col items-center bg-gray-900 text-white py-2">
      <template v-for="(tool, index) in activeTools" :key="index">
        <button
          class="tool-icon hover:bg-gray-700 p-1 rounded"
          @click="tool.onClick"
        >
          <span v-html="tool.icon" class="text-lg"></span>
        </button>
      </template>
    </aside>
    
    <!-- Main Content and Resizable Right Panel -->
    <div class="flex-1 flex">
      <router-view class="flex-1 p-4 bg-white" />

      <div class="resize-container">
        <div class="resize-bar" @mousedown="startResize"></div>
        <div class="right-panel p-2 bg-gray-200">
          <component :is="activeRightPanel" />
        </div>
      </div>
    </div>
  </div>
      
      <StatusBar :status="appStatus" :version="appVersion" />
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, computed, watch } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  import MenuBar from './main_menu/MenuBar.vue'
  import StatusBar from './StatusBar.vue'
  import { useStatusStore } from '@/stores/statusStore';
  import { toolbarConfig } from '@/components/toolbarConfig';
  import { useRoute } from 'vue-router';
import { screensConfig } from '@/screensConfig';
  
  export default defineComponent({
    name: 'MainWindow',
    components: {
      MenuBar,
      StatusBar
    },
    data() {
    return {
      tools: toolbarConfig, // Load toolbar tools from configuration
      isResizing: false,
    };
    },
    methods: {
    startResize() {
      this.isResizing = true;
      document.addEventListener("mousemove", this.resizePanel);
      document.addEventListener("mouseup", this.stopResize);
    },
    resizePanel(event: MouseEvent) {
      if (this.isResizing) {
        const newWidth = window.innerWidth - event.clientX;
        const rightPanel = document.querySelector(".right-panel") as HTMLElement;
        if (newWidth > 100 && newWidth < 600) {
          rightPanel.style.width = `${newWidth}px`;
        }
      }
    },
    stopResize() {
      this.isResizing = false;
      document.removeEventListener("mousemove", this.resizePanel);
      document.removeEventListener("mouseup", this.stopResize);
    },
  },
    setup() {
      const appStatus = ref('Ready')
      const appVersion = ref('1.0.0')
      const greetMsg = ref('')
      const statusStore = useStatusStore();

      const route = useRoute();
      const activeScreen = computed(() => screensConfig[route.name as string]);
      const activeTools = computed(() => activeScreen.value?.tools || []);
      const activeRightPanel = computed(() => activeScreen.value?.rightPanel || "DefaultRightPanel");

      const updateStatus = (message: string) => {
        statusStore.setStatus(message)
      }
  
      async function greet() {
        greetMsg.value = await invoke('greet', { name: 'Tauri' });

        statusStore.setStatus(greetMsg.value);
      }
  
      return {
        appStatus,
        appVersion,
        greetMsg,
        greet,
        updateStatus,
        activeTools,
        activeRightPanel
      }
    }
  })
  </script>

<style scoped>
/* Styling for the sidebar */
.sidebar {
  width: 48px;
  background-color: #1a1a1a;
}

.tool-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  margin-bottom: 4px; /* Minimal spacing between icons */
  border: none;
  background-color: transparent;
  cursor: pointer;
  transition: background-color 0.2s;
}

.tool-icon:focus {
  outline: none;
}

/* Resizable right panel and border */
.resize-container {
  display: flex;
  align-items: stretch;
}

.resize-bar {
  width: 5px;
  cursor: ew-resize;
  background-color: #ccc;
}

.right-panel {
  width: 300px; /* Default width */
  min-width: 100px;
  max-width: 600px;
  background-color: #f3f3f3;
  border-left: 1px solid #ddd;
}
</style>