import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('../pages/Login.vue'),
    meta: { noAuth: true },
  },
  {
    path: '/',
    name: 'Dashboard',
    component: () => import('../pages/Dashboard.vue'),
  },
  {
    path: '/generate',
    name: 'Generate',
    component: () => import('../pages/Generate.vue'),
  },
  {
    path: '/tasks',
    name: 'Tasks',
    component: () => import('../pages/Tasks.vue'),
  },
  {
    path: '/result/:id',
    name: 'Result',
    component: () => import('../pages/Result.vue'),
  },
  {
    path: '/xhs',
    name: 'XhsAnalytics',
    component: () => import('../pages/XhsAnalytics.vue'),
  },
  {
    path: '/admin',
    name: 'Admin',
    component: () => import('../pages/Admin.vue'),
    meta: { requiresAdmin: true },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('token')
  if (to.meta.noAuth) {
    next()
  } else if (!token) {
    next('/login')
  } else {
    next()
  }
})

export default router