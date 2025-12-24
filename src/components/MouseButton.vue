<script setup lang="ts">
import { CONTAINER, TEXT } from '@/styles/classes';

type ButtonType = 'Left' | 'Middle' | 'Right';

const selectedButton = defineModel<ButtonType>();
const BUTTONS = ['Left', 'Middle', 'Right'] as const;
</script>

<template>
  <div :class="CONTAINER.section">
    <label :class="[TEXT.label, 'block mb-3']">Mouse Button</label>
    <div class="grid grid-cols-3 gap-2">
      <button
        v-for="button in BUTTONS"
        :key="button"
        @click="selectedButton = button"
        :class="[
          'py-2.5 rounded-lg font-medium text-sm transition-all duration-200',
          selectedButton === button 
            ? 'bg-slate-600 text-white shadow-lg' 
            : 'bg-slate-700/50 text-slate-400 hover:bg-slate-700 hover:text-white'
        ]"
      >
        {{ button }}
      </button>
    </div>
  </div>
</template>
