<script setup lang="ts">
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
    <svg v-if="!isRunning" class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
      <path d="M8 5v14l11-7z"></path>
    </svg>
    <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
      <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z"></path>
    </svg>
    {{ isRunning ? 'STOP' : 'START' }}
  </button>
</template>
