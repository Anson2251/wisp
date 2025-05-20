<script setup lang="ts">
import { NEquation } from 'naive-ui';
import CopyIndicator from './CopyIndicator.vue';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import {computed} from 'vue';

const props = defineProps<{
  value: string
  katexOptions: any
  style: string | Record<string, string>
  inline: boolean
}>();

const styleLocal = computed(() => {
  if (typeof props.style === 'string') {
    return `${props.style}; display: ${props.inline ? 'inline-block' : 'block'};`;
  } else {
    return {
      ...props.style,
      display: props.inline ? 'inline-block' : 'block',
    };
  }
});
</script>

<template>
  <n-equation v-if="inline" :value="value" :katex-options="katexOptions" :style="styleLocal" />
  <copy-indicator v-else text-to-copy="Click to copy" text-copied="Copied" :on-copy="async () => await writeText(value)">
    <template #trigger>
      <n-equation :value="value" :katex-options="katexOptions" :style="styleLocal" />
    </template>
    <span>Click to copy</span>
  </copy-indicator>
</template>
