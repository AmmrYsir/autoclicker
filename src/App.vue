<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const isRunning = ref(false);
const intervalMs = ref(100);
const statusMessage = ref("");
const isDarkMode = ref(false);

const displayInterval = computed(() => {
  if (intervalMs.value < 1000) return `${intervalMs.value}ms`;
  return `${(intervalMs.value / 1000).toFixed(1)}s`;
});

onMounted(async () => {
  isRunning.value = await invoke("is_clicking");
  isDarkMode.value = window.matchMedia("(prefers-color-scheme: dark)").matches;
});

async function toggleAutoclicker() {
  try {
    if (isRunning.value) {
      await invoke("stop_clicking");
      statusMessage.value = "Autoclicker stopped";
      isRunning.value = false;
    } else {
      await invoke("start_clicking", { intervalMs: intervalMs.value });
      statusMessage.value = `Clicking at ${displayInterval.value} interval`;
      isRunning.value = true;
    }
  } catch (error) {
    statusMessage.value = `Error: ${error}`;
  }
}

function handleIntervalChange(event: Event) {
  const target = event.target as HTMLInputElement;
  intervalMs.value = parseInt(target.value) || 100;
}

function resetToDefault() {
  intervalMs.value = 100;
  statusMessage.value = "Reset to default interval";
}
</script>

<template>
  <div 
    :class="[
      'min-h-screen w-full transition-colors duration-300',
      isDarkMode 
        ? 'bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900' 
        : 'bg-gradient-to-br from-blue-50 via-white to-indigo-50'
    ]"
  >
    <!-- Animated Background Elements -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <div 
        class="absolute top-0 left-0 w-96 h-96 rounded-full opacity-20 blur-3xl animate-pulse-glow"
        :class="isDarkMode ? 'bg-blue-500' : 'bg-blue-400'"
      ></div>
      <div 
        class="absolute bottom-0 right-0 w-96 h-96 rounded-full opacity-20 blur-3xl animate-pulse-glow"
        :class="isDarkMode ? 'bg-indigo-500' : 'bg-indigo-400'"
        style="animation-delay: 1s"
      ></div>
    </div>

    <!-- Main Content -->
    <div class="relative z-10 min-h-screen flex flex-col items-center justify-center px-4 py-8 sm:py-12">
      <!-- Header -->
      <div class="mb-12 text-center animate-fade-in">
        <h1 class="text-5xl sm:text-6xl md:text-7xl font-black mb-3 tracking-tighter">
          <span class="bg-clip-text text-transparent bg-gradient-to-r from-blue-600 to-indigo-600 dark:from-blue-400 dark:to-indigo-400">
            🖱️ AutoClick
          </span>
        </h1>
        <p class="text-base sm:text-lg text-gray-600 dark:text-gray-300 font-medium">
          Effortless Mouse Automation
        </p>
      </div>

      <!-- Main Card -->
      <div class="w-full max-w-md animate-scale-in">
        <div 
          class="relative rounded-3xl p-8 sm:p-10 backdrop-blur-xl border transition-all duration-300"
          :class="[
            isRunning
              ? isDarkMode
                ? 'bg-red-900/10 border-red-500/30 shadow-glow-red'
                : 'bg-red-50/40 border-red-300/40 shadow-glow-red'
              : isDarkMode
                ? 'bg-slate-800/40 border-slate-700/50'
                : 'bg-white/60 border-white/80'
          ]"
        >
          <!-- Running Indicator -->
          <div v-if="isRunning" class="absolute -top-4 left-1/2 -translate-x-1/2">
            <div class="flex items-center gap-2 bg-gradient-to-r from-red-500 to-pink-500 text-white px-4 py-2 rounded-full font-semibold text-sm shadow-lg animate-bounce-slow">
              <span class="w-2 h-2 bg-white rounded-full animate-pulse"></span>
              CLICKING
            </div>
          </div>

          <!-- Interval Control -->
          <div class="mb-8">
            <div class="flex justify-between items-baseline mb-4">
              <label class="text-sm font-bold uppercase tracking-widest text-gray-700 dark:text-gray-300">
                Click Interval
              </label>
              <span 
                class="text-2xl sm:text-3xl font-black transition-colors duration-300"
                :class="isRunning ? 'text-red-600 dark:text-red-400' : 'text-blue-600 dark:text-blue-400'"
              >
                {{ displayInterval }}
              </span>
            </div>
            
            <!-- Slider -->
            <input
              type="range"
              :value="intervalMs"
              @change="handleIntervalChange"
              :disabled="isRunning"
              min="50"
              max="5000"
              step="10"
              class="w-full h-3 rounded-full appearance-none bg-gradient-to-r from-blue-400 to-indigo-400 dark:from-blue-500 dark:to-indigo-500 slider-thumb cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed transition-opacity"
            />
            
            <!-- Range Labels -->
            <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-3 font-medium">
              <span>50ms (Fast)</span>
              <span>5000ms (Slow)</span>
            </div>
          </div>

          <!-- Main Toggle Button -->
          <button
            @click="toggleAutoclicker"
            :disabled="false"
            class="w-full mb-4 py-4 px-6 rounded-2xl font-bold text-lg uppercase tracking-wider transition-all duration-300 transform hover:scale-105 active:scale-95 shadow-xl focus:outline-none focus:ring-4"
            :class="[
              isRunning
                ? 'bg-gradient-to-r from-red-500 to-pink-500 text-white hover:shadow-glow-red focus:ring-red-300 dark:focus:ring-red-700'
                : 'bg-gradient-to-r from-blue-500 to-indigo-600 text-white hover:shadow-glow focus:ring-blue-300 dark:focus:ring-blue-700'
            ]"
          >
            <span class="inline-block transition-transform">
              {{ isRunning ? '⏹ Stop Clicking' : '▶ Start Clicking' }}
            </span>
          </button>

          <!-- Secondary Button -->
          <button
            @click="resetToDefault"
            :disabled="isRunning"
            class="w-full py-3 px-6 rounded-xl font-semibold text-sm uppercase tracking-wider transition-all duration-300 border-2 disabled:opacity-50 disabled:cursor-not-allowed"
            :class="[
              isDarkMode
                ? 'border-slate-600 text-slate-300 hover:bg-slate-700'
                : 'border-gray-300 text-gray-700 hover:bg-gray-100'
            ]"
          >
            ↻ Reset to Default
          </button>
        </div>
      </div>

      <!-- Status Message -->
      <transition
        enter-active-class="animate-slide-in-top"
        leave-active-class="animate-fade-in"
      >
        <div 
          v-if="statusMessage"
          class="mt-8 px-6 py-4 rounded-2xl font-semibold text-center max-w-md transition-all duration-300 shadow-lg"
          :class="[
            isRunning
              ? isDarkMode
                ? 'bg-red-900/30 border border-red-500/50 text-red-200'
                : 'bg-red-100/80 border border-red-300 text-red-700'
              : isDarkMode
                ? 'bg-green-900/30 border border-green-500/50 text-green-200'
                : 'bg-green-100/80 border border-green-300 text-green-700'
          ]"
        >
          {{ statusMessage }}
        </div>
      </transition>

      <!-- Stats Card -->
      <div 
        class="mt-12 w-full max-w-md rounded-2xl p-6 sm:p-8 backdrop-blur-xl border transition-all duration-300 animate-fade-in"
        :class="isDarkMode
          ? 'bg-slate-800/40 border-slate-700/50'
          : 'bg-white/60 border-white/80'"
        style="animation-delay: 0.2s"
      >
        <h3 class="text-sm font-bold uppercase tracking-widest text-gray-600 dark:text-gray-400 mb-6">
          Current Settings
        </h3>
        
        <div class="space-y-4">
          <!-- Status Row -->
          <div class="flex items-center justify-between">
            <span class="text-gray-700 dark:text-gray-300 font-medium">Status</span>
            <span 
              class="px-4 py-2 rounded-full font-bold text-sm"
              :class="[
                isRunning
                  ? 'bg-red-100 dark:bg-red-900/50 text-red-700 dark:text-red-300'
                  : 'bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300'
              ]"
            >
              {{ isRunning ? 'ACTIVE' : 'IDLE' }}
            </span>
          </div>

          <!-- Interval Row -->
          <div class="flex items-center justify-between">
            <span class="text-gray-700 dark:text-gray-300 font-medium">Interval</span>
            <span class="font-bold text-indigo-600 dark:text-indigo-400">{{ displayInterval }}</span>
          </div>

          <!-- Info Message -->
          <div class="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700">
            <p class="text-xs text-gray-500 dark:text-gray-400">
              ⚠️ Use responsibly. Click interval is measured in milliseconds. Minimum 50ms for fastest clicking.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Custom slider styling */
input[type="range"] {
  -webkit-appearance: none;
  appearance: none;
  background: linear-gradient(to right, rgb(96, 165, 250), rgb(129, 140, 248));
  outline: none;
}

input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: linear-gradient(135deg, rgb(59, 130, 246), rgb(99, 102, 241));
  cursor: pointer;
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
  transition: all 0.3s ease;
  border: 3px solid white;
}

input[type="range"]::-webkit-slider-thumb:hover {
  transform: scale(1.2);
  box-shadow: 0 0 20px rgba(59, 130, 246, 0.8);
}

input[type="range"]::-moz-range-thumb {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: linear-gradient(135deg, rgb(59, 130, 246), rgb(99, 102, 241));
  cursor: pointer;
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
  transition: all 0.3s ease;
  border: 3px solid white;
}

input[type="range"]::-moz-range-thumb:hover {
  transform: scale(1.2);
  box-shadow: 0 0 20px rgba(59, 130, 246, 0.8);
}

input[type="range"]:disabled::-webkit-slider-thumb {
  background: linear-gradient(135deg, rgb(156, 163, 175), rgb(107, 114, 128));
  box-shadow: 0 0 5px rgba(107, 114, 128, 0.3);
  cursor: not-allowed;
}

input[type="range"]:disabled::-moz-range-thumb {
  background: linear-gradient(135deg, rgb(156, 163, 175), rgb(107, 114, 128));
  box-shadow: 0 0 5px rgba(107, 114, 128, 0.3);
  cursor: not-allowed;
}

/* Smooth transitions */
button {
  position: relative;
  overflow: hidden;
}

button::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.5);
  transform: translate(-50%, -50%);
  transition: width 0.6s, height 0.6s;
  pointer-events: none;
}

button:active::before {
  width: 300px;
  height: 300px;
}
</style>