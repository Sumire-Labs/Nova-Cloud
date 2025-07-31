import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import RegisterView from '../views/RegisterView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: LoginView,
      meta: { transition: 'slide-prev' },
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView,
      meta: { transition: 'slide-next' },
    },
  ],
})

export default router
