<script setup lang="ts">
interface Props {
  isRunning: boolean;
}

defineProps<Props>();
</script>

<template>
  <div class="flex items-center justify-between mb-6">
    <div class="flex items-center gap-2">
      <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122"></path>
      </svg>
      <h1 class="text-xl font-bold text-white">AutoClicker</h1>
    </div>
    <div class="flex items-center gap-2">
      <div :class="['w-2 h-2 rounded-full', isRunning ? 'bg-green-400 animate-pulse' : 'bg-emerald-400']"></div>
      <span class="text-xs font-medium text-emerald-400 uppercase tracking-wider">
        {{ isRunning ? 'RUNNING' : 'READY' }}
      </span>
    </div>
  </div>
</template>
