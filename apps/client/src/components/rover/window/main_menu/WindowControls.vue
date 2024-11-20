<template>
  <div class="flex">
    <button @click="minimizeWindow" class="px-4 py-1 hover:bg-[#505050] focus:outline-none transition duration-150 ease-in-out">
      &#8211;
    </button>
    <button @click="toggleMaximizeWindow" class="px-4 py-1 hover:bg-[#505050] focus:outline-none transition duration-150 ease-in-out">
      &#9633;
    </button>
    <button @click="closeWindow" class="px-4 py-1 hover:bg-[#e81123] focus:outline-none transition duration-150 ease-in-out">
      &#10005;
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow();

export default defineComponent({
  name: 'WindowControls',
  
  setup() {
    const minimizeWindow = async () => {
      await appWindow.minimize();
    };

    const toggleMaximizeWindow = async () => {
      await appWindow.toggleMaximize();
    };

    const closeWindow = async () => {
      await appWindow.close();
    };

    return {
      minimizeWindow,
      toggleMaximizeWindow,
      closeWindow
    }
  }
})
</script>