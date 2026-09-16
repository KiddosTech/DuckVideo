<template>
  <svg
    aria-hidden="true"
    class="simple-brand-icon"
    role="img"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path :d="icon.path" fill="currentColor" />
  </svg>
</template>

<script setup lang="ts">
import type { SimpleIcon } from 'simple-icons';

defineProps<{
  icon: SimpleIcon;
}>();
</script>
