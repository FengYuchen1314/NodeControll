import { createRouter, createWebHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: () => import('../views/DashboardPage.vue'),
    },
    {
      path: '/system',
      name: 'system',
      component: () => import('../views/SystemPage.vue'),
    },
    {
      path: '/setup',
      name: 'setup',
      component: () => import('../views/SetupPage.vue'),
    },
  ],
})
