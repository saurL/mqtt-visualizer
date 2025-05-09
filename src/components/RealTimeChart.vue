<!-- RealtimeChart.vue -->
<template>
  <div class="chart-container">
    <p>Graphique pour {{ props.data_name }}</p>
    <apexchart
      ref="chartRef"
      type="line"
      :options="chartOptions"
      :series="series"
      height="350"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { useMyStore } from "@/stores/store";

const chartRef = ref();
const MAX_POINTS = 1000; // Nombre maximum de points à afficher

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

const store = useMyStore();
const series = ref([
  { name: "Valeurs", data: store.map[props.data_name].slice(-MAX_POINTS) },
]); // Initialiser avec les dernières valeurs

// Surveiller les changements dans le store
watchEffect(() => {
  console.log("dans watchEffect", props.data_name);
  console.log(" le store:", store.map[props.data_name]);
  console.log("serie length", series.value[0].data.length);
  const newData = store.map[props.data_name] || [];
  if (newData.length > 0) {
    // Prendre le dernier point ajouté
    const lastPoint = newData[newData.length - 1];
    // Vérifier que le point est nouveau pour éviter les doublons
    if (
      series.value[0].data.length === 0 ||
      series.value[0].data[series.value[0].data.length - 1][0] !== lastPoint[0]
    ) {
      series.value[0].data.push(lastPoint);
      if (series.value[0].data.length > MAX_POINTS) {
        // Supprimer le point le plus ancien si on dépasse le maximum
        series.value[0].data = series.value[0].data.slice(-100);
        console.log("On surpirme");
      }
    }
  }
});

const chartOptions = {
  chart: {
    id: "realtime",
    animations: {
      enabled: true,
      easing: "linear",
      dynamicAnimation: { speed: 500 },
    },
    toolbar: { show: false },
    zoom: { enabled: false },
    background: "transparent",
  },
  stroke: {
    curve: "smooth",
    width: 2,
    colors: ["#007bff"],
  },
  xaxis: {
    type: "datetime",
    range: 1 * 60 * 1000, // 1 minute
    tickAmount: 6,
    labels: {
      style: { colors: "#666", fontSize: "10px" },
      datetimeUTC: false,
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
    show: false,
  },
  markers: {
    size: 0,
  },
  tooltip: {
    enabled: true,
    theme: "light",
    x: { show: false },
  },
};
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
