<template>
  <Card>
    <CardTitle>{{ title }}</CardTitle>
    <CardContent> {{ displayed_data }} {{ symbol }} </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import { useMyStore } from "@/stores/store";
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";
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
const displayed_data = ref("");
store.map[props.data_name];

listen(props.data_name, (event: any) => {
  displayed_data.value = event.payload as string; // Mettre à jour le tableau dans le store
});
</script>
<style scoped>
.coloredData {
  text-align: right;
  min-width: 68px;
}
</style>
