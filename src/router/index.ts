import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/home/index.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView,
        },
        {
            path: '/settings',
            name: 'settings',
            component: () => import('../views/settings.vue'),
        },
    ],
})

export default router
