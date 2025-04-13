<script lang="ts" setup>
import { NCard, NInput, NButton, NSpace } from 'naive-ui'
import MessageBubble from './MessageBubble.vue'
import { useChatStore } from '../stores/chat'
import { useMarkdown } from '../composables/useMarkdown'
import { useAutoScroll } from '../composables/useAutoScroll'
import { useOpenAI } from '../composables/useOpenAI'

const { provideMarkdownProcessor } = useMarkdown()
provideMarkdownProcessor()

const chatStore = useChatStore()
const { scrollToBottom, setupAutoScroll } = useAutoScroll('message-bubbles')
const { streamResponse } = useOpenAI()

setupAutoScroll(chatStore.messages)

async function sendMessage() {
  if (!chatStore.userInput.trim()) return

  const userMessage = {
    text: chatStore.userInput,
    sender: 'user' as const,
    timestamp: new Date()
  }
  chatStore.addMessage(userMessage)
  chatStore.clearUserInput()

  const botMessage = {
    text: '',
    sender: 'bot' as const,
    timestamp: new Date()
  }
  chatStore.addMessage(botMessage)
  const botMessageIndex = chatStore.messages.length - 1

  await streamResponse(
    chatStore.messages.map(msg => ({
      role: msg.sender === 'user' ? 'user' : 'assistant',
      content: msg.text
    })),
    (chunk) => {
      chatStore.messages[botMessageIndex].text += chunk
    }
  )

  scrollToBottom(false)
}
</script>

<template>
  <div class="chat-container">
    <n-card class="messages-container" content-style="overflow: auto; min-height: 0;">
      <n-space vertical style="height: 100%" id="message-bubbles">
        <MessageBubble v-for="(message, index) in chatStore.messages" :key="index" :text="message.text"
          :sender="message.sender" :timestamp="message.timestamp" />
      </n-space>
    </n-card>

    <div class="input-container">
      <n-input v-model:value="chatStore.userInput" placeholder="Type your message..." @keyup.enter="sendMessage"
        clearable />
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
