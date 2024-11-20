<template>
    <div>
      <h3 class="font-semibold">Unit Status</h3>
      <sh-select label="Status" :options="['Available', 'Busy', 'Off Duty']" />
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({});
  </script>
  