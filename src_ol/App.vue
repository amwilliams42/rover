<template>
  <div class="h-screen flex flex-col bg-gray-100">
    <!-- Top Menu -->
    <div data-tauri-drag-region class="titlebar">
    <TopMenuBar data-tauri-drag-region=""/>
    </div>

    <div class="flex flex-1">
      <!-- Left Toolbar -->
      <LeftToolbar />

      <div class="flex-1 flex flex-col">
        <!-- Top Filters / Info Bar -->
        <TopFiltersBar />

        <!-- Main Canvas / Content Area -->
        <MainCanvas />
      </div>

      <!-- Right Side Panel with Tools/Forms -->
      <RightPanel />
    </div>
  </div>
</template>

<script>
import TopMenuBar from './components/TopMenuBar.vue';
import LeftToolbar from './components/LeftToolbar.vue';
import TopFiltersBar from './components/TopFiltersBar.vue';
import MainCanvas from './components/MainCanvas.vue';
import RightPanel from './components/RightPanel.vue';

export default {
  components: {
    TopMenuBar,
    LeftToolbar,
    TopFiltersBar,
    MainCanvas,
    RightPanel,
  },
};
</script>

<style scoped>
/* Optional: Root-level styles */
</style>
