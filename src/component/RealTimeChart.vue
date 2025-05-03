<!-- RealtimeChart.vue -->
<template>
  <div class="chart-container">
    <p>Graphique pour {{ props.data_name }}</p>
    <apexchart
      type="line"
      :options="chartOptions"
      :series="series"
      height="350"
    />
  </div>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";

const props = defineProps({
  data_name: {
    type: String,
    required: true,
  },
  symbol: {
    type: String,
    default: undefined,
  },
});

const series = ref<{ name: string; data: [number, number][] }[]>([
  {
    name: "Valeurs",
    data: [],
  },
]);

const chartOptions = {
  chart: {
    id: "realtime",
    animations: {
      enabled: true,
      easing: "linear",
      dynamicAnimation: { speed: 500 }, // un peu plus rapide
    },
    toolbar: { show: false },
    zoom: { enabled: false },
    background: "transparent", // fond transparent
  },
  stroke: {
    curve: "smooth", // rend les lignes courbées
    width: 2,
    colors: ["#007bff"],
  },
  xaxis: {
    type: "datetime",
    range: 1 * 60 * 1000,
    tickAmount: 6,
    labels: {
      style: { colors: "#666", fontSize: "10px" },
    },
    axisBorder: { show: false },
    axisTicks: { show: false },
  },
  yaxis: {
    labels: {
      formatter: (val: number) => val.toFixed(0),
      style: { colors: "#666", fontSize: "10px" },
    },
    axisBorder: { show: false },
    axisTicks: { show: false },
  },
  grid: {
    show: false, // ou true pour des lignes fines
  },
  markers: {
    size: 0, // pas de points sur les courbes
  },
  tooltip: {
    enabled: true,
    theme: "light",
    x: { show: false },
  },
};

listen(props.data_name, (event) => {
  console.log(event);
  const now = new Date().getTime();

  // Ajouter le nouveau point
  series.value[0].data.push([now, event.payload as number]);

  // Filtrer les données pour ne conserver que les 2 dernières minutes
});
</script>

<style scoped>
.chart-container {
  width: 100%;
  max-width: 300px;
  margin: auto;
  padding: 20px;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}
</style>
