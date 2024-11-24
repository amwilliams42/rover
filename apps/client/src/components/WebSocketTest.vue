<!-- src/components/WebSocketDebug.vue -->
<template>
  <div class="websocket-debug">
    <h2>WebSocket Debug</h2>
    
    <!-- Connection Status -->
    <div :class="['status-indicator', connectionStatus]">
      Connection Status: {{ connectionStatus }}
    </div>
    
    <!-- Ping Button -->
    <div class="controls">
      <button 
        @click="sendPing" 
        :disabled="isPinging || connectionStatus !== 'connected'"
      >
        {{ isPinging ? 'Sending Ping...' : 'Send Ping' }}
      </button>
    </div>

    <!-- Message Log -->
    <div class="messages">
      <div v-for="(msg, index) in messages" :key="index" class="message">
        {{ msg }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const messages = ref([])
const isPinging = ref(false)
const connectionStatus = ref('connecting')
let unlistenMessage = null
let unlistenStatus = null

onMounted(async () => {
  // Listen for WebSocket messages
  unlistenMessage = await listen('ws-message', (event) => {
    messages.value.push(`Received: ${event.payload}`)
  })

  // Listen for connection status
  unlistenStatus = await listen('ws-status', (event) => {
    connectionStatus.value = event.payload
  })
})

onUnmounted(async () => {
  if (unlistenMessage) await unlistenMessage()
  if (unlistenStatus) await unlistenStatus()
})

async function sendPing() {
  if (connectionStatus.value !== 'connected') return

  try {
    isPinging.value = true
    await invoke('send_ping')
    messages.value.push('Sent: ping')
  } catch (error) {
    messages.value.push(`Error: ${error}`)
    console.error('Failed to send ping:', error)
  } finally {
    isPinging.value = false
  }
}
</script>

<style scoped>
.websocket-debug {
  padding: 1rem;
  max-width: 600px;
  margin: 0 auto;
}

.status-indicator {
  padding: 0.5rem;
  margin-bottom: 1rem;
  border-radius: 4px;
  text-align: center;
  font-weight: bold;
}

.status-indicator.connected {
  background-color: #4CAF50;
  color: white;
}

.status-indicator.connecting {
  background-color: #FFC107;
  color: black;
}

.status-indicator.disconnected {
  background-color: #f44336;
  color: white;
}

.controls {
  margin-bottom: 1rem;
}

button {
  padding: 0.5rem 1rem;
  background-color: #2196F3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.messages {
  background-color: #f5f5f5;
  padding: 1rem;
  border-radius: 4px;
  max-height: 400px;
  overflow-y: auto;
}

.message {
  padding: 0.5rem;
  margin-bottom: 0.5rem;
  background-color: white;
  border-radius: 4px;
  box-shadow: 0 1px 2px rgba(0,0,0,0.1);
}
</style>