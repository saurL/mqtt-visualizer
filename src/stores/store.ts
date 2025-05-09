import { defineStore } from "pinia";
import { reactive } from "vue";

export const useMyStore = defineStore("myStore", {
  state: () => ({
    map: reactive({}) as Record<string, [number, number][]>, // Define the type explicitly
  }),
  actions: {
    addData(key: string, value: [number, number]) {
      if (!this.map[key]) {
        this.map[key] = [];
      }
      this.map[key].push(value);
    },
  },
});