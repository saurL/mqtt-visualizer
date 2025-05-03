<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";
import ChangingValue from "./component/ChangingValue.vue";
import RealTimeChart from "./component/RealTimeChart.vue";
const dataNames = ref<string[]>([]);

onMounted(async () => {
  try {
    dataNames.value = await invoke<string[]>("get_data_names");
    console.log("Data names:", dataNames.value);
  } catch (error) {
    console.error("Failed to fetch data names:", error);
  }
});
</script>

<template>
  <main class="container">
    <RealTimeChart
      v-for="dataName in dataNames"
      :key="dataName"
      :data_name="dataName"
    ></RealTimeChart>
    <ChangingValue
      v-for="dataName in dataNames"
      :key="dataName"
      :data_name="dataName"
    ></ChangingValue>
  </main>
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
