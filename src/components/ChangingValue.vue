<template>
  <Card>
    <CardTitle>{{ title }}</CardTitle>
    <CardContent> {{ roundToTwo(displayed_data) }} {{ symbol }} </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import { computed } from "vue";
import { useMyStore } from "@/stores/store";
const props = defineProps({
  title: {
    type: String,
    required: true,
  },
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
const store = useMyStore();
const displayed_data = computed(() => {
  const data = store.map[props.data_name];
  if (data && data.length > 0) {
    return data[data.length - 1][1]; // Get the last value
  }
  return 0; // Default value if no data is available
});
store.map[props.data_name];
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
