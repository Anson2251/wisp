<script lang="ts" setup>
import { ref, onMounted, watch, provide } from 'vue'
import { remark } from 'remark'
import html from 'remark-html'
import { rehype } from 'rehype'
import rehypeMermaid from 'rehype-mermaid'
import { NCard, NInput, NButton, NSpace } from 'naive-ui'
import MessageBubble from './MessageBubble.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const markdownProcessor = {
  remark: remark().use(html),
  rehype: rehype().use(rehypeMermaid, {
    strategy: 'img-svg',
    errorFallback: () => {}
  })
}

provide('markdownProcessor', markdownProcessor)

onMounted(() => {
  watch(messages, () => {
    autoScrollToBottom();
  }, { deep: true })
})

const autoScrollThrottle = 48

function autoScrollToBottom(withThrottle = true) {
  if (!messageContainer) {
    const container = (document.getElementById('message-bubbles') as HTMLDivElement);
    if (container) {
      messageContainer = (container.parentElement as HTMLDivElement);
    }
  }
  if (messageContainer) {
    if(messageContainer.scrollTop + messageContainer.clientHeight === messageContainer.scrollHeight) return
    // if user's viewport not stick to the bottom, ignore
    if((messageContainer.scrollTop + messageContainer.clientHeight < messageContainer.scrollHeight - autoScrollThrottle) && withThrottle) return
    console.log(messageContainer.scrollTop + messageContainer.clientHeight, messageContainer.scrollHeight)
    messageContainer.scrollTo({
      top: messageContainer.scrollHeight,
      behavior: 'smooth'
    });
  }
}

interface Message {
  text: string
  sender: 'user' | 'bot'
  timestamp: Date
}

let messageContainer: HTMLDivElement | null = null;

const messages = ref<Message[]>([])
const userInput = ref('')

async function sendMessage() {
  if (!userInput.value.trim()) return

  const userMessage = {
    text: userInput.value,
    sender: 'user' as const,
    timestamp: new Date()
  }
  messages.value.push(userMessage)
  userInput.value = ''

  // Add a placeholder bot message
  const botMessage = {
    text: '',
    sender: 'bot' as const,
    timestamp: new Date()
  }
  messages.value.push(botMessage)
  const botMessageIndex = messages.value.length - 1

  // Listen for streaming chunks
  const unlisten = await listen<string>('openai_stream_chunk', (event) => {
    messages.value[botMessageIndex].text += event.payload
  })

  autoScrollToBottom(false)

  try {
    const conversationHistory = messages.value.map(msg => ({
      role: msg.sender === 'user' ? 'user' : 'assistant',
      content: msg.text
    }))

    await invoke('ask_openai_stream', {
      messages: conversationHistory
    })
  } finally {
    unlisten()
  }
}
</script>

<template>
  <div class="chat-container">
    <n-card class="messages-container" content-style="overflow: auto; min-height: 0;">
      <n-space vertical style="height: 100%" id="message-bubbles">
        <MessageBubble v-for="(message, index) in messages" :key="index" :text="message.text" :sender="message.sender"
          :timestamp="message.timestamp" />
      </n-space>
    </n-card>

    <div class="input-container">
      <n-input v-model:value="userInput" placeholder="Type your message..." @keyup.enter="sendMessage" clearable />
      <n-button type="primary" @click="sendMessage">Send</n-button>
    </div>
  </div>
</template>


<style scoped>
.chat-container {
  display: grid;
  grid-template-rows: 1fr auto;
  height: 100vh;
}

.messages-container {
  min-width: 0;
  min-height: 0;
}

.input-container {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 8px;
  padding: 8px;
}
</style>
