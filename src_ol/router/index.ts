// src/router/index.ts
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import DashboardView from '../views/DashboardView.vue';
import UnitsView from '../views/UnitsView.vue';
import CallsView from '../views/CallsView.vue';

const routes: Array<RouteRecordRaw> = [
  { path: '/', component: DashboardView },
  { path: '/units', component: UnitsView },
  { path: '/calls', component: CallsView },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
