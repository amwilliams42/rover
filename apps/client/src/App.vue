<template>
  <div class="app">
    <MainWindow />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import MainWindow from './components/rover/window/MainWindow.vue'

export default defineComponent({
  name: 'App',
  components: {
    MainWindow
  }
})
</script>