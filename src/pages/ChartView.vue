<script setup lang="ts">
import RealTimeChart from "@/components/RealTimeChart.vue";
import Navigation from "@/components/Navigation.vue";
import { useMyStore } from "@/stores/store";
import { computed } from "vue";

const store = useMyStore();
const MAX_POINTS = 100;

Object.keys(store.map).forEach((key) => {
  if (store.map[key].length > MAX_POINTS) {
    store.map[key] = store.map[key].slice(-MAX_POINTS);
  }
});

const dataNames = computed(() => Object.keys(store.map));
console.log("Data names:", dataNames.value);
</script>

<template>
  <Navigation></Navigation>
  <div class="container">
    <RealTimeChart
      v-for="dataName in dataNames"
      :key="dataName"
      :data_name="dataName"
    ></RealTimeChart>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  width: 100vw;
  background-color: #f0f0f0;
  flex-wrap: wrap;
}
</style>
