<script setup lang="ts">
import { TEXT, BADGE } from '@/styles/classes';
</script>

<template>
  <div class="text-center flex items-center justify-center gap-2">
    <span :class="TEXT.labelSmall">Hotkey</span>
    <span :class="BADGE.hotkey">F8</span>
  </div>
</template>
