<script setup lang="ts">
type ButtonType = 'Left' | 'Middle' | 'Right';

const selectedButton = defineModel<ButtonType>();
</script>

<template>
  <div class="mb-6 p-4 bg-slate-700/30 rounded-lg border border-slate-600/50">
    <label class="block text-xs font-semibold text-slate-400 uppercase tracking-wider mb-3">Mouse Button</label>
    <div class="grid grid-cols-3 gap-2">
      <button
        v-for="button in ['Left', 'Middle', 'Right'] as const"
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
