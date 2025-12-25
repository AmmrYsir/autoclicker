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
    <div :class="[CONTAINER.flexRow, 'gap-3']">
      <MousePointer2 class="w-7 h-7 text-blue-400" />
      <div :class="[CONTAINER.flexRow, 'gap-2 items-baseline']">
        <h1 class="text-2xl font-extrabold bg-gradient-to-r from-blue-400 to-cyan-400 bg-clip-text text-transparent">
          AutoClicker
        </h1>
        <span class="px-2 py-0.5 text-[10px] font-semibold bg-blue-500/20 text-blue-300 rounded border border-blue-500/30 uppercase tracking-wider">
          v0.5
        </span>
      </div>
    </div>
    <div :class="[CONTAINER.flexRow, 'gap-2']">
      <div :class="[BADGE.status, isRunning ? COLORS.running : COLORS.ready]"></div>
      <span :class="[TEXT.labelSmall, isRunning ? 'text-orange-400' : 'text-emerald-400']">
        {{ isRunning ? 'RUNNING' : 'READY' }}
      </span>
    </div>
  </div>
</template>
