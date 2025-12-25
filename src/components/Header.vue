<script setup lang="ts">
import { MousePointer2 } from 'lucide-vue-next';
import { CONTAINER, TEXT, BADGE, COLORS } from '@/styles/classes';

interface Props {
  isRunning: boolean;
  clickCount: number;
}

defineProps<Props>();
</script>

<template>
  <div :class="CONTAINER.header">
    <div :class="[CONTAINER.flexRow, 'gap-2']">
      <MousePointer2 class="w-6 h-6 text-blue-400" />
      <h1 class="text-xl font-bold text-white">AutoClicker v.0.5</h1>
    </div>
    <div :class="[CONTAINER.flexRow, 'gap-2']">
      <div :class="[BADGE.status, isRunning ? COLORS.running : COLORS.ready]"></div>
      <span :class="[TEXT.labelSmall, isRunning ? 'text-orange-400' : 'text-emerald-400']">
        {{ isRunning ? 'RUNNING' : 'READY' }}
      </span>
      <span v-if="isRunning" :class="[TEXT.labelSmall, 'text-slate-400 ml-2']">
        | {{ clickCount.toLocaleString() }} clicks
      </span>
    </div>
  </div>
</template>
