<template>
  <p>{{ data_name }} :</p>
  <p class="">{{ roundToTwo(displayed_data) }} {{ symbol }}</p>
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
  maxValue: {
    type: Number,
    default: 100,
  },
  dangerStartValue: {
    type: Number,
    default: undefined,
  },
  dangerMaxValue: {
    type: Number,
    default: undefined,
  },
  minValue: {
    type: Number,
    default: 0,
  },
});
const displayed_data = ref(0);
listen(props.data_name, (event) => {
  console.log(event);
  displayed_data.value = event.payload as number; // Traitement de l'événement
});
function roundToTwo(num: number) {
  return Math.round(num * 100) / 100;
}
</script>
<style scoped>
.coloredData {
  text-align: right;
  min-width: 68px;
}
</style>
