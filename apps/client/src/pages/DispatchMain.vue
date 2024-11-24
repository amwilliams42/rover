<template>
    <div class="container mx-auto py-4">
      <h1 class="text-2xl font-bold mb-4">Dispatch Center</h1>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Card>
          <CardHeader>
            <CardTitle>Pending Calls</CardTitle>
          </CardHeader>
          <CardContent>
            <div class="space-y-2">
              <div v-for="call in pendingCalls" :key="call.id" class="flex items-center justify-between p-2 bg-gray-50 rounded-md">
                <span>{{ call.description }}</span>
                <Button @click="assignCall(call)" size="sm">Assign</Button>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Units on Calls</CardTitle>
          </CardHeader>
          <CardContent>
            <div class="space-y-2">
              <div v-for="unit in activeUnits" :key="unit.id" class="flex items-center justify-between p-2 bg-gray-50 rounded-md">
                <span>{{ unit.name }} - {{ unit.currentCall }}</span>
                <Button @click="completeCall(unit)" variant="outline" size="sm">Complete</Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
      <WebSocketTest />
    </div>

  </template>
  
  <script lang="ts" setup>
  import { ref } from 'vue';
  import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
  import { Button } from '@/components/ui/button';
  import WebSocketTest from '../components/WebSocketTest.vue';
  
  interface Call {
    id: number;
    description: string;
  }
  
  interface Unit {
    id: number;
    name: string;
    currentCall: string;
  }
  
  const pendingCalls = ref<Call[]>([
    { id: 1, description: 'Emergency at 123 Main St' },
    { id: 2, description: 'Traffic accident on Highway 1' },
  ]);
  
  const activeUnits = ref<Unit[]>([
    { id: 1, name: 'Unit 1', currentCall: 'Fire at 456 Elm St' },
    { id: 2, name: 'Unit 2', currentCall: 'Medical emergency at 789 Oak Ave' },
  ]);
  
  const assignCall = (call: Call) => {
    // Implement assign call logic
    console.log('Assigning call', call);
  };
  
  const completeCall = (unit: Unit) => {
    // Implement complete call logic
    console.log('Completing call for unit', unit);
  };
  </script>