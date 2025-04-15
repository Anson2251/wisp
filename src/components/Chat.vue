<script lang="ts" setup>
import { NInput, NButton } from 'naive-ui'
import MessageBubble from './MessageBubble.vue'
import AutoScrollWrapper from './AutoScrollWrapper.vue'
import { useChatStore } from '../stores/chat'
import { useMarkdown } from '../composables/useMarkdown'
import { useOpenAI } from '../composables/useOpenAI'
import { onMounted, ref } from 'vue'

const { provideMarkdownProcessor } = useMarkdown()
provideMarkdownProcessor()

const chatStore = useChatStore()
const { streamResponse } = useOpenAI()
const autoScrollWrapper = ref()


async function sendMessage() {
  if (!chatStore.userInput.trim()) return

  const userMessage = {
    text: chatStore.userInput,
    sender: 'user' as const,
    timestamp: new Date()
  }
  await chatStore.addMessage(userMessage)
  chatStore.clearUserInput()

  const botMessage = {
    text: '',
    sender: 'bot' as const,
    timestamp: new Date()
  }
  await chatStore.addMessage(botMessage, false)
  const botMessageId = chatStore.messages[chatStore.messages.length - 1].id

  await streamResponse(
    chatStore.messages.map(msg => ({
      role: msg.sender === 'user' ? 'user' : 'assistant',
      content: msg.text
    })),
    (chunk) => {
      const message = chatStore.messages.find(m => m.id === botMessageId)
      if (message) {
        message.text += chunk
      }
      autoScrollWrapper.value?.scrollToBottom()
    },
    () => {
      chatStore.saveMessage({
        ...(chatStore.messages.find(m => m.id === botMessageId)!),
        timestamp: new Date()
      })
      // const message = chatStore.messages.find(m => m.id === botMessageId)
      // if (message) {
      //   message.text += "\n\n ` `" // TODO: to trigger the final render for the last block
      // }
    }
  )

  autoScrollWrapper.value?.scrollToBottom(false)
}

onMounted(() => {
  chatStore.loadMessages().then(() => {
    setTimeout(() => autoScrollWrapper.value?.scrollToBottom(false), 100)
  })
})
</script>

<template>
  <div class="chat-container">
    <div class="messages-container">
      <AutoScrollWrapper ref="autoScrollWrapper" :auto-scroll="true">
        <div class="bubble-container">
        <MessageBubble v-for="message in chatStore.messages" :key="message.id" :text="message.text"
          :sender="message.sender" :timestamp="message.timestamp" :id="message.id"/>
        </div>
      </AutoScrollWrapper>
    </div>

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

.bubble-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 8px;
}
</style>
