<script lang="ts" setup>
import { NInput, NButton } from 'naive-ui'
import MessageBubble from './MessageBubble.vue'
import AutoScrollWrapper from './AutoScrollWrapper.vue'
import { useChatStore } from '../stores/chat'
import { useOpenAI } from '../composables/useOpenAI'
import { onMounted, ref } from 'vue'
import debounce from 'lodash/debounce'

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
    text: 'Generating ...',
    sender: 'bot' as const,
    timestamp: new Date()
  }
  await chatStore.addMessage(botMessage, false)
  const botMessageId = chatStore.messages[chatStore.messages.length - 1].id

  const updateMessage = (text: string) => {
    const message = chatStore.messages.find(m => m.id === botMessageId)
    if (message) {
      message.text = text
    }
  }
  const updateBubbleText = debounce(updateMessage, 50)
  autoScrollWrapper.value?.scrollToBottom(false)

  let responseText = ""
  await streamResponse(
    chatStore.messages.map(msg => ({
      role: msg.sender === 'user' ? 'user' : 'assistant',
      content: msg.text
    })),
    (chunk) => {
      responseText += chunk
      updateBubbleText(responseText)
      autoScrollWrapper.value?.scrollToBottom()
    },
    () => {
      const message = chatStore.messages.find(m => m.id === botMessageId)
      if(!message) return
      message.text = responseText
      chatStore.saveMessage({
        ...message,
        timestamp: new Date()
      })

      // const message = chatStore.messages.find(m => m.id === botMessageId)
      // if (message) {
      //   message.text += "\n\n ` `" // TODO: to trigger the final render for the last block
      // }
    }
  )
}

onMounted(() => {
  chatStore.loadMessages().then(() => {
    setTimeout(() => autoScrollWrapper.value?.scrollToBottom(false), 1000)
  })
})
</script>

<template>
  <div class="chat-container">
    <div class="messages-container">
      <AutoScrollWrapper ref="autoScrollWrapper" :auto="true" :throttle="256" :smooth="true">
        <div class="bubble-container">
          <MessageBubble v-for="message in chatStore.messages" :key="message.id" :text="message.text"
            :sender="message.sender" :timestamp="message.timestamp" :id="message.id" />
        </div>
      </AutoScrollWrapper>
    </div>

    <div class="input-container">
      <n-input v-model:value="chatStore.userInput" placeholder="Type your message..." @keyup.enter="sendMessage"
        clearable round type="textarea"/>
      <n-button type="primary" @click="sendMessage" round>Send</n-button>
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
  align-items: center;
}

.bubble-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 8px;
}
</style>
