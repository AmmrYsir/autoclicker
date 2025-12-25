<script setup lang="ts">
import { Play, Pause, Timer } from 'lucide-vue-next';
import { ref, watch } from 'vue';

interface Props {
  isRunning: boolean;
  isDelayed: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  toggle: [];
}>();

const countdown = ref(3);
let countdownInterval: number | null = null;

watch(() => props.isDelayed, (delayed) => {
  if (delayed) {
    countdown.value = 3;
    countdownInterval = window.setInterval(() => {
      countdown.value--;
      if (countdown.value <= 0 && countdownInterval) {
        clearInterval(countdownInterval);
        countdownInterval = null;
      }
    }, 1000);
  } else if (countdownInterval) {
    clearInterval(countdownInterval);
    countdownInterval = null;
  }
});

const handleClick = () => {
  if (!props.isDelayed) {
    emit('toggle');
  }
};

const buttonClasses = (isRunning: boolean, isDelayed: boolean) => [
  'w-full py-3.5 rounded-xl font-bold text-white uppercase tracking-wide transition-all duration-200 flex items-center justify-center gap-2 shadow-lg',
  isDelayed
    ? 'bg-slate-600 cursor-not-allowed opacity-75'
    : isRunning
    ? 'bg-red-500 hover:bg-red-600 shadow-red-500/50'
    : 'bg-blue-500 hover:bg-blue-600 shadow-blue-500/50'
];
</script>

<template>
  <button
    @click="handleClick"
    :class="buttonClasses(isRunning, isDelayed)"
    :disabled="isDelayed"
  >
    <template v-if="isDelayed">
      <Timer class="w-5 h-5" />
      STARTING IN {{ countdown }}s
    </template>
    <template v-else>
      <Play v-if="!isRunning" class="w-5 h-5" :fill="'currentColor'" />
      <Pause v-else class="w-5 h-5" :fill="'currentColor'" />
      {{ isRunning ? 'STOP' : 'START' }}
    </template>
  </button>
</template>
