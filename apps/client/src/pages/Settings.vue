<template>
  <div class="container mx-auto py-8">
    <h1 class="text-2xl font-bold mb-6">Settings</h1>
    <Tabs default-value="account" class="w-full">
      <TabsList>
        <TabsTrigger value="account">Account</TabsTrigger>
        <TabsTrigger value="appearance">Appearance</TabsTrigger>
        <TabsTrigger value="notifications">Notifications</TabsTrigger>
      </TabsList>
      <TabsContent value="account">
        <Card>
          <CardHeader>
            <CardTitle>Account Settings</CardTitle>
            <CardDescription>Manage your account details and preferences.</CardDescription>
          </CardHeader>
          <CardContent>
            <form @submit.prevent="saveAccountSettings">
              <div class="space-y-4">
                <div class="space-y-2">
                  <Label for="email">Email</Label>
                  <Input id="email" v-model="email" type="email" placeholder="Enter your email" />
                </div>
                <div class="space-y-2">
                  <Label for="name">Name</Label>
                  <Input id="name" v-model="name" type="text" placeholder="Enter your name" />
                </div>
                <Button type="submit">Save Changes</Button>
              </div>
            </form>
          </CardContent>
        </Card>
      </TabsContent>
      <!-- Add content for other tabs -->
    </Tabs>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

const email = ref('');
const name = ref('');

const saveAccountSettings = () => {
  // Implement save logic here
  console.log('Saving account settings', { email: email.value, name: name.value });
};
</script>