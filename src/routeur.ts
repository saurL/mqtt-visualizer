// src/router.ts
import { createRouter, createWebHistory } from 'vue-router'

import NumberView from '@/pages/NumberView.vue'
import ChartView from '@/pages/ChartView.vue'

const routes = [
    {
        path: '/',
        redirect: '/numbers'
      },
  { path: '/numbers', name: 'Home', component: NumberView },
  { path: '/charts', name: 'About', component: ChartView }
]

export const router = createRouter({
  history: createWebHistory(), // ou `createWebHashHistory()` si besoin
  routes
})
