<template>
  <div class="bg-[#007acc] text-white text-xs flex items-center justify-between px-2 py-1">
    <div>{{ status.message }}</div>
    <div>{{ status.appVersion }}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { useStatusStore } from '@/stores/statusStore'

export default defineComponent({
  name: 'StatusBar',
  setup() {
    const status = useStatusStore()

    return {
      status
    }
  }
})
</script>