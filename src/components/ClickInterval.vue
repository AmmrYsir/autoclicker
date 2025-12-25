<script setup lang="ts">
import { Clock, Minus, Plus } from 'lucide-vue-next';
import { CONTAINER, TEXT, BUTTON, INPUT } from '../styles/classes';

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
  <div :class="CONTAINER.section">
    <div :class="[CONTAINER.flexRow, 'justify-between mb-3']">
      <label :class="TEXT.label">Click Interval</label>
      <Clock class="w-4 h-4 text-slate-500" />
    </div>
    <div :class="[CONTAINER.flexRow, 'gap-2']">
      <button 
        @click="decrementInterval"
        :class="[BUTTON.small, 'flex justify-center items-center']"
      >
        <Minus :class="BUTTON.iconSmall" />
      </button>
      <div class="flex-1 relative">
        <input 
          type="number" 
          :value="clickInterval"
          @input="handleIntervalInput"
          min="10"
          max="10000"
          :class="INPUT.number"
        />
        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-slate-500 font-medium uppercase">ms</span>
      </div>
      <button 
        @click="incrementInterval"
        :class="[BUTTON.small, 'flex justify-center items-center']"
      >
        <Plus :class="BUTTON.iconSmall" />
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
