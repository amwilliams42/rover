import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Settings from '@/pages/Settings.vue';
import DispatchMain from '@/pages/DispatchMain.vue';

const routes: Array<RouteRecordRaw> = [
  { path: '/', name: 'dashboard', component: DispatchMain },
  { path: '/settings', name: 'Settings', component: Settings },
  { path: '/dispatch', name: 'DispatchMain', component: DispatchMain },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;