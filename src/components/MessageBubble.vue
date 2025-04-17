<script setup lang="ts">
import { NAvatar, NIcon, NButton, NFlex, NButtonGroup, useDialog } from 'naive-ui'
import { Chat24Regular, Person24Regular, Copy16Regular, Delete16Regular } from '@vicons/fluent'
import MarkdownRenderer from './MarkdownRenderer.vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { useChatStore } from '../stores/chat'

const { deleteMessage } = useChatStore()
const dialog = useDialog()

const props = defineProps<{
  text: string
  sender: 'user' | 'bot'
  timestamp: Date
  id: string
}>()

const copyMessage = async () => {
  await writeText(props.text)
  const bubble = document.querySelector(`.message-bubble[id="${props.id}"]`)
  if (bubble) {
    bubble.classList.add('copied')
    setTimeout(() => bubble.classList.remove('copied'), 500)
  }
}

const removeMessage = () => {
  dialog.warning({
    title: 'Delete Message',
    content: 'Are you sure you want to delete this message?',
    positiveText: 'Delete',
    negativeText: 'Cancel',
    onPositiveClick: () => {
      deleteMessage(props.id)
    }
  })
}
</script>

<template>
  <n-flex align="start" :wrap-item="false" :style="{
    flexDirection: sender === 'user' ? 'row-reverse' : 'row',
    alignItems: 'flex-start',
    marginBottom: '12px'
  }">
    <n-avatar style="position: sticky; top: 0;">
      <n-icon :component="sender === 'bot' ? Chat24Regular : Person24Regular" />
    </n-avatar>
    <div class="message-bubble" :class="sender" :id="id">
      <div class="content">
        <MarkdownRenderer :text="text" />
      </div>
      <div class="footer">
        <n-button-group class="button-group">
          <n-button quaternary :onclick="copyMessage" circle>
            <template #icon>
              <n-icon :component="Copy16Regular" />
            </template>
          </n-button>
          <n-button quaternary :onclick="removeMessage" circle type="error">
            <template #icon>
              <n-icon :component="Delete16Regular" />
            </template>
          </n-button>
        </n-button-group>
        <span class="timestamp">{{ timestamp.toLocaleTimeString() }}</span>
      </div>
    </div>
  </n-flex>
</template>

<style scoped>
.message-bubble {
  max-width: 80%;
  width: fit-content;
  border-radius: 16px;
  padding: 12px 16px;
  margin-left: 12px;
  margin-right: 12px;
  transition: all 0.2s ease;
  position: relative;
  margin-bottom: 16px;

  --footer-buttons-opacity: 0;
}

.message-bubble:hover {
  --footer-buttons-opacity: 1;
}

.message-bubble.user {
  background: linear-gradient(135deg, #e3f2fd, #bbdefb);
  box-shadow: 0 2px 8px rgba(38, 78, 100, 0.2), 0 1px 4px rgba(13, 62, 90, 0.2);
  margin-left: auto;
}

.message-bubble.bot {
  background: linear-gradient(135deg, #f5f5f5, #eeeeee);
  box-shadow: 0 2px 8px rgba(76, 76, 76, 0.2), 0 1px 4px rgba(38, 38, 38, 0.1);
  margin-right: auto;
}

.message-bubble.copied {
  transform-origin: bottom;
  animation: flash 0.5s ease;
}

@keyframes flash {
  0% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.9; transform: scale(0.98); }
  100% { opacity: 1; transform: scale(1); }
}

.content {
  overflow: auto;
}

.footer {
  position: absolute;
  bottom: -38px;
  left: 2px;
  padding-top: 8px;

  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 8px;
  width: 100%;
  opacity: var(--footer-buttons-opacity);
  transition: opacity .1s ease-in;
}

.timestamp {
  font-size: 0.8em;
  margin-left: 16px;
  margin-right: 8px;
}
</style>
