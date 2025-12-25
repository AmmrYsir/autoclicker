<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import Header from './components/Header.vue';
import ClickInterval from './components/ClickInterval.vue';
import MouseButton from './components/MouseButton.vue';
import StartButton from './components/StartButton.vue';
import Hotkey from './components/Hotkey.vue';

const clickInterval = ref(100);
const selectedButton = ref<'Left' | 'Middle' | 'Right'>('Left');
const isRunning = ref(false);
const clickCount = ref(0);

let counterInterval: number | null = null;

// Poll click count when running
const startCounterPolling = () => {
  if (counterInterval) return;
  
  counterInterval = window.setInterval(async () => {
    try {
      clickCount.value = await invoke<number>('get_click_count');
    } catch (error) {
      console.error('Failed to get click count:', error);
    }
  }, 100); // Update every 100ms
};

const stopCounterPolling = () => {
  if (counterInterval) {
    clearInterval(counterInterval);
    counterInterval = null;
  }
};

// Watch isRunning to start/stop polling
watch(isRunning, (running) => {
  if (running) {
    startCounterPolling();
  } else {
    stopCounterPolling();
  }
});

const toggleClicking = async () => {
  if (isRunning.value) {
    await invoke('stop_clicking');
    isRunning.value = false;
    clickCount.value = 0; // Reset counter when stopped
  } else {
    try {
      await invoke('start_clicking', { 
        intervalMs: clickInterval.value,
        button: selectedButton.value
      });
      isRunning.value = true;
    } catch (error) {
      console.error('Failed to start autoclicker:', error);
    }
  }
};

const HOTKEY = 'F8';
const HOTKEY_COOLDOWN_MS = 300;
let hotkeyCooling = false;
let keyListener: ((event: KeyboardEvent) => void) | null = null;

onMounted(async () => {
  let registered = false;

  try {
    await register(HOTKEY, () => {
      if (hotkeyCooling) return; // ignore OS key repeat while held down
      hotkeyCooling = true;
      toggleClicking();
      setTimeout(() => {
        hotkeyCooling = false;
      }, HOTKEY_COOLDOWN_MS);
    });
    registered = true;
  } catch (err) {
    console.warn('Global shortcut registration failed, falling back to window listener', err);
  }

  if (!registered) {
    keyListener = (event: KeyboardEvent) => {
      if (event.key === HOTKEY) {
        event.preventDefault();
        if (hotkeyCooling) return;
        hotkeyCooling = true;
        toggleClicking();
        setTimeout(() => {
          hotkeyCooling = false;
        }, HOTKEY_COOLDOWN_MS);
      }
    };
    window.addEventListener('keydown', keyListener);
  }
});

onUnmounted(async () => {
  stopCounterPolling();
  
  try {
    await unregister(HOTKEY);
  } catch (err) {
    console.warn('Failed to unregister global shortcut', err);
  }

  if (keyListener) {
    window.removeEventListener('keydown', keyListener);
  }
});
</script>

<template>
  <div class="h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 p-6 flex flex-col gap-6 select-none">
    <!-- Header -->
    <Header :is-running="isRunning" :click-count="clickCount" />

    <!-- Main Content -->
    <div class="flex flex-col gap-2 flex-1">
      <ClickInterval v-model="clickInterval" />
      <MouseButton v-model="selectedButton" />
      <StartButton :is-running="isRunning" @toggle="toggleClicking" />
    </div>

    <!-- Hotkey at Bottom -->
    <Hotkey />
  </div>
</template>
<style>
/* Global styles */
</style>