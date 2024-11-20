import { defineStore } from 'pinia'

export const useStatusStore = defineStore('status', {
  state: () => ({
    message: 'Ready',
    appVersion: 'v1.0.0'
  }),
  actions: {
    setStatus(message: string) {
      this.message = message
    },
    setAppVersion(version: string) {
      this.appVersion = version
    }
  }
})