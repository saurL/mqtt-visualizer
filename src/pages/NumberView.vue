<script setup lang="ts">
import ChangingValue from "@/components/ChangingValue.vue";
import ChangingValueString from "@/components/ChangingValueString.vue";
import Navigation from "@/components/Navigation.vue";
import { useMyStore } from "@/stores/store";
import { ref } from "vue";
import { watchEffect } from "vue";

const store = useMyStore();
const data_names = ref(Object.keys(store.map).filter((key) => key !== "ipv4"));

watchEffect(() => {
  data_names.value = Object.keys(store.map).filter((key) => key !== "ipv4");
});
</script>

<template>
  <Navigation></Navigation>
  <div class="container">
    <ChangingValueString
      :title="'Adresse IP'"
      :data_name="'ipv4'"
    ></ChangingValueString>
    <ChangingValue
      v-for="data_name in data_names"
      :title="'Voltage'"
      :data_name="data_name"
    ></ChangingValue>
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
