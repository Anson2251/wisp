<script setup lang="ts">
import { NCard, NAvatar, NIcon, NButton, NFlex } from 'naive-ui'
import { Chat24Regular, Person24Regular, Copy16Regular, Delete16Regular } from '@vicons/fluent'
import MarkdownRenderer from './MarkdownRenderer.vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { useChatStore } from '../stores/chat'

const { deleteMessage } = useChatStore()

const props = defineProps<{
  text: string
  sender: 'user' | 'bot'
  timestamp: Date
  id: string
}>()

let onContextMenu = () => { }

const copyMessage = () => {
  writeText(props.text)
}

const removeMessage = () => {
  deleteMessage(props.id)
}
</script>

<template>
  <n-flex align="start" :wrap-item="false" :style="{
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
    }" :oncontextmenu="onContextMenu">
      <MarkdownRenderer :text="text" />
      <template #footer>
        <n-flex align="center">
          <n-button quaternary :onclick="copyMessage" circle>
            <template #icon>
              <n-icon>
                <Copy16Regular />
              </n-icon>
            </template>
          </n-button>
          <n-button quaternary :onclick="removeMessage" circle type="error">
            <template #icon>
              <n-icon>
                <Delete16Regular />
              </n-icon>
            </template>
          </n-button>
          <span style="padding-left: 32px;">{{ timestamp.toLocaleTimeString() }}</span>
        </n-flex>

      </template>
    </n-card>
  </n-flex>
</template>

<style scoped>
/* All markdown styling moved to MarkdownRenderer.vue */
</style>
