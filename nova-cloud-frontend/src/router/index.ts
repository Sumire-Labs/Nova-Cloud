import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import RegisterView from '../views/RegisterView.vue'
import MainView from '../views/MainView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: LoginView,
      meta: { transition: 'shared-axis-backward' },
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView,
      meta: { transition: 'shared-axis-forward' },
    },
    {
      path: '/main',
      name: 'main',
      component: MainView,
      meta: { requiresAuth: true, transition: 'fade' }, // Added auth requirement
    },
  ],
})

export default router
