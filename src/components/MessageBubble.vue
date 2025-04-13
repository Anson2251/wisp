<script setup lang="ts">
import { NCard, NAvatar, NIcon } from 'naive-ui'
import { Chat24Regular, Person24Regular } from '@vicons/fluent'
import MarkdownRenderer from './MarkdownRenderer.vue'

defineProps<{
  text: string
  sender: 'user' | 'bot'
  timestamp: Date
}>()
</script>

<template>
  <n-space align="start" :wrap-item="false" :style="{
    flexDirection: sender === 'user' ? 'row-reverse' : 'row',
    alignItems: 'flex-start'
  }">
    <n-avatar style="position: sticky; top: 0;">
      <n-icon :component="sender === 'bot' ? Chat24Regular : Person24Regular" />
    </n-avatar>
    <n-card :bordered="sender === 'user'" size="small" padding="false" :style="{
      marginLeft: sender === 'user' ? 'auto' : '0',
      maxWidth: '80%',
      backgroundColor: sender === 'user' ? '#f0f7ff' : 'transparent',
      width: 'fit-content',
      padding: '0px 16px',
      overflow: 'auto'
    }">
      <MarkdownRenderer :text="text" />
      <template #footer>
        {{ timestamp.toLocaleTimeString() }}
      </template>
    </n-card>
  </n-space>
</template>

<style scoped>
/* All markdown styling moved to MarkdownRenderer.vue */
</style>
