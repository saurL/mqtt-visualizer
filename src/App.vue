<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useMyStore } from "@/stores/store";
const dataNames = ref<string[]>([]);
const store = useMyStore();

onMounted(async () => {
  try {
    dataNames.value = await invoke<string[]>("get_data_names");
    console.log("Data names:", dataNames.value);
    for (const dataname of dataNames.value) {
      store.map[dataname] = []; // Initialize the array for each data name
      listen(dataname, (event) => {
        if (dataname == "ipv4") {
          return;
        }
        const now = new Date().getTime();

        store.map[dataname].push([now, event.payload as number]); // Mettre à jour le tableau dans le store
      });
    }
  } catch (error) {
    console.error("Failed to fetch data names:", error);
  }
});
</script>

<template>
  <main class="w-full h-full flex flex-col items-center justify-center">
    <router-view />
  </main>
</template>

<style scoped></style>
