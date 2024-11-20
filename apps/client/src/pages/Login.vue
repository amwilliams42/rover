<template>
    <div class="flex items-center justify-center min-h-screen bg-gray-100">
      <Card class="w-full max-w-md">
        <CardHeader>
          <CardTitle>Login</CardTitle>
          <CardDescription>Enter your credentials to access the dispatch system</CardDescription>
        </CardHeader>
        <CardContent>
          <form @submit.prevent="handleLogin">
            <div class="space-y-4">
              <div class="space-y-2">
                <Label for="username">Username</Label>
                <Input id="username" v-model="username" type="text" placeholder="Enter your username" />
              </div>
              <div class="space-y-2">
                <Label for="password">Password</Label>
                <Input id="password" v-model="password" type="password" placeholder="Enter your password" />
              </div>
              <Button type="submit" class="w-full">Log in</Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  </template>
  
  <script lang="ts" setup>
  import { ref } from 'vue';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
  import { Input } from '@/components/ui/input';
  import { Label } from '@/components/ui/label';
  import { Button } from '@/components/ui/button';
  
  const username = ref('');
  const password = ref('');
  
  const handleLogin = () => {
    // Implement login logic here
    console.log('Login attempted', { username: username.value, password: password.value });
  };
  </script>