<script setup lang="ts">
import { NAvatar, NIcon, NButton, NFlex, NButtonGroup, useDialog, useThemeVars } from 'naive-ui'
import { Chat24Regular, Person24Regular, Copy16Regular, Delete16Regular, Edit16Regular, ChevronLeft16Regular, ChevronRight16Regular } from '@vicons/fluent'
import MarkdownRenderer from './MarkdownRenderer.vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { MessageRole } from '../libs/types'
import { useChatStore } from '../stores/chat'
import { ref, computed } from 'vue'
import MessageBubbleEditor from './MessageBubbleEditor.vue'
import { mixColours } from '../utils/colour'

const { deleteMessage } = useChatStore()
const dialog = useDialog()
const theme = useThemeVars()

const borderColor = computed(() =>
  props.sender === MessageRole.User
    ? 'transparent'
    : theme.value.borderColor
)

const backgroundColor = computed(() =>
  props.sender === MessageRole.User
    ? mixColours(theme.value.primaryColor, theme.value.baseColor, 0.3)
    : theme.value.cardColor
)

const props = defineProps<{
  text: string
  sender: MessageRole
  timestamp: Date
  id: string
  over?: boolean
  hasPrevious?: boolean
  hasNext?: boolean
}>()

const emit = defineEmits(['previous', 'next'])

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
    onPositiveClick: async () => {
      await deleteMessage(props.id)
      console.log('Message deleted:', props.id)
    }
  })
}

const showEditorModal = ref(false)
const editMessage = () => {
  showEditorModal.value = true
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
    <div class="message-bubble" :class="sender" :id="id" :style="{
      'background-color': backgroundColor,
      'border-radius': theme.borderRadius,
      'box-shadow': theme.boxShadow1,
      'border': `1px solid ${borderColor}`,
    }">
      <div class="content">
        <MarkdownRenderer :text="text" :over="over" />
      </div>
      <div class="footer" :style="{
        justifyContent: sender === 'user' ? 'flex-end' : 'flex-start',
      }">
        <div class="footer-content">
          <n-flex align="center">
            <n-button-group class="button-group">
              <n-button quaternary :onclick="copyMessage" size="tiny">
                <template #icon>
                  <n-icon :component="Copy16Regular" :size="16"/>
                </template>
              </n-button>
              <n-button quaternary :onclick="removeMessage" type="error" size="tiny">
                <template #icon>
                  <n-icon :component="Delete16Regular" :size="18"/>
                </template>
              </n-button>
              <n-button quaternary :onclick="editMessage" size="tiny">
                <template #icon>
                  <n-icon :component="Edit16Regular" :size="16"/>
                </template>
              </n-button>
            </n-button-group>
            <n-button-group class="nav-group" v-if="hasPrevious || hasNext">
              <n-button quaternary @click="emit('previous')" size="tiny" :disabled="!hasPrevious">
                <template #icon>
                  <n-icon :component="ChevronLeft16Regular" :size="16"/>
                </template>
              </n-button>
              <n-button quaternary @click="emit('next')" size="tiny" :disabled="!hasNext">
                <template #icon>
                  <n-icon :component="ChevronRight16Regular" :size="16"/>
                </template>
              </n-button>
            </n-button-group>
          </n-flex>
          <span class="timestamp" :style="{
            color: theme.textColorBase,
          }">{{ timestamp.toLocaleTimeString() }}</span>
        </div>
      </div>
    </div>
  </n-flex>
  <MessageBubbleEditor v-model:show="showEditorModal" :id="id" />
</template>

<style scoped>
.message-bubble {
  max-width: 80%;
  width: fit-content;
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
  color: white;
  /* background: linear-gradient(135deg, var(--to), var(--from)); */
  margin-left: auto;
}

.message-bubble.bot {
  /* background: linear-gradient(135deg, var(--to), var(--from)); */
  margin-right: auto;
}

.message-bubble.copied {
  transform-origin: bottom;
  animation: flash 0.5s ease;
}

@keyframes flash {
  0% {
    opacity: 1;
    transform: scale(1);
  }

  50% {
    opacity: 0.9;
    transform: scale(0.98);
  }

  100% {
    opacity: 1;
    transform: scale(1);
  }
}

.content {
  overflow: auto;
}

.footer {
  position: absolute;
  bottom: -30px;
  left: 2px;
  padding-top: 8px;

  display: flex;
  align-items: center;
  overflow: visible;
  margin-top: 8px;
  width: 100%;
  opacity: var(--footer-buttons-opacity);
  transition: opacity .1s ease-in;
}

.footer-content {
  width: 100%;
  min-width: fit-content;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.nav-group {
  margin-left: 8px;
}

.timestamp {
  font-size: 0.8em;
  margin-left: 16px;
  margin-right: 8px;

  width: fit-content;
}
</style>
