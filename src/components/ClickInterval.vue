<script setup lang="ts">
import { ref } from 'vue';

const clickInterval = defineModel<number>();

const decrementInterval = () => {
  if (clickInterval.value! > 10) {
    clickInterval.value = Math.max(10, clickInterval.value! - 10);
  }
};

const incrementInterval = () => {
  if (clickInterval.value! < 10000) {
    clickInterval.value = Math.min(10000, clickInterval.value! + 10);
  }
};

const handleIntervalInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const value = parseInt(target.value);
  if (!isNaN(value) && value >= 10 && value <= 10000) {
    clickInterval.value = value;
  }
};
</script>

<template>
  <div class="mb-6 p-4 bg-slate-700/30 rounded-lg border border-slate-600/50">
    <div class="flex items-center justify-between mb-3">
      <label class="text-xs font-semibold text-slate-400 uppercase tracking-wider">Click Interval</label>
      <svg class="w-4 h-4 text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
    </div>
    <div class="flex items-center gap-2">
      <button 
        @click="decrementInterval"
        class="w-10 h-10 bg-slate-700 hover:bg-slate-600 text-white rounded-lg transition-all duration-200 flex items-center justify-center"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"></path>
        </svg>
      </button>
      <div class="flex-1 relative">
        <input 
          type="number" 
          :value="clickInterval"
          @input="handleIntervalInput"
          min="10"
          max="10000"
          class="w-full bg-slate-900/50 text-white text-center text-2xl font-bold py-2 rounded-lg border border-slate-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-slate-500 font-medium uppercase">ms</span>
      </div>
      <button 
        @click="incrementInterval"
        class="w-10 h-10 bg-slate-700 hover:bg-slate-600 text-white rounded-lg transition-all duration-200 flex items-center justify-center"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
/* Remove spinner from number input */
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

input[type="number"] {
  -moz-appearance: textfield;
}
</style>
