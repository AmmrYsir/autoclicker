<script setup lang="ts">
import { Play, Pause } from 'lucide-vue-next';

interface Props {
  isRunning: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
  toggle: [];
}>();

const handleClick = () => {
  emit('toggle');
};

const buttonClasses = (isRunning: boolean) => [
  'w-full py-3.5 rounded-xl font-bold text-white uppercase tracking-wide transition-all duration-200 flex items-center justify-center gap-2 shadow-lg',
  isRunning
    ? 'bg-red-500 hover:bg-red-600 shadow-red-500/50'
    : 'bg-blue-500 hover:bg-blue-600 shadow-blue-500/50'
];
</script>

<template>
  <button
    @click="handleClick"
    :class="buttonClasses(isRunning)"
  >
    <Play v-if="!isRunning" class="w-5 h-5" :fill="'currentColor'" />
    <Pause v-else class="w-5 h-5" :fill="'currentColor'" />
    {{ isRunning ? 'STOP' : 'START' }}
  </button>
</template>
