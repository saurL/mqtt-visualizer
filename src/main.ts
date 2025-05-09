import { createApp } from "vue";
import App from "./App.vue";
import VueApexCharts from 'vue3-apexcharts'
import { createPinia } from 'pinia'
import { router } from './routeur'  
const pinia = createPinia()

createApp(App).use(pinia).use(router).use(VueApexCharts).mount("#app");
