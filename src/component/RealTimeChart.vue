<!-- RealtimeChart.vue -->
<template>
  <p>Graphique pour {{ props.data_name }}</p>
  <apexchart
    type="line"
    :options="chartOptions"
    :series="series"
    height="350"
  />
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
      dynamicAnimation: { speed: 1000 },
    },
  },
  xaxis: {
    type: "datetime",
    range: 2 * 60 * 1000, // 2 minutes glissantes
    tickAmount: 6,
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
