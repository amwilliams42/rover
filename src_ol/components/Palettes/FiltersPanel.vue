<template>
    <div>
      <h3 class="font-semibold">Filters</h3>
      <sh-select label="Priority" :options="['All', 'High', 'Medium', 'Low']" />
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({});
  </script>