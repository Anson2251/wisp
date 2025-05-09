<script lang="ts" setup>
import { NInput, NButton } from 'naive-ui'
import MessageBubble from './MessageBubble.vue'
import AutoScrollWrapper from './AutoScrollWrapper.vue'
import { useChatStore } from '../stores/chat'
import { useOpenAI } from '../composables/useOpenAI'
import { onMounted, ref } from 'vue'
import debounce from 'lodash/debounce'
import { Message, MessageRole } from '../libs/types'

const chatStore = useChatStore()
const { streamResponse } = useOpenAI()
const autoScrollWrapper = ref()


async function sendMessage() {
  if (!chatStore.userInput.trim()) return

  const userMessage: Omit<Message, 'id'> = {
    text: chatStore.userInput,
    sender: MessageRole.User,
    timestamp: Math.round((new Date()).getTime() / 1000),
  }
  const userMessageId = await chatStore.addMessage(userMessage)
  chatStore.clearUserInput()

  const botMessage: Omit<Message, 'id'> = {
    text: 'Generating ...',
    sender: MessageRole.Assistant,
    timestamp: Math.round((new Date()).getTime() / 1000),
  }

  const botMessageId = await chatStore.addMessage(botMessage, userMessageId)

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
      if (!message) return
      message.text = responseText
      chatStore.updateMessage(botMessageId, responseText)
    }
  )
}

onMounted(async () => {
  try {
    const conversations = await chatStore.listConversations();
    let conversationId = ""

    if (conversations.length == 0) {
      conversationId = await chatStore.createConversation("Untitled Conversation", "")
    }
    else conversationId = conversations[0].id

    chatStore.currentConversationId = conversationId
    console.log('Current conversation id: ', conversationId)

    chatStore.loadMessages(conversationId).then(() => {
      setTimeout(() => autoScrollWrapper.value?.scrollToBottom(false), 1000)
    })
  }
  catch (error) {
    console.error('Error loading messages:', error)
  }
})
</script>

<template>
  <div class="chat-container">
    <div class="messages-container">
      <AutoScrollWrapper ref="autoScrollWrapper" :auto="true" :throttle="256" :smooth="true">
        <div class="bubble-container">
          <MessageBubble v-for="message in chatStore.messages" :key="message.id" :text="message.text"
            :sender="message.sender" :timestamp="new Date(message.timestamp * 1000)" :id="message.id" />
        </div>
      </AutoScrollWrapper>
    </div>

    <div class="input-container">
      <n-input v-model:value="chatStore.userInput" placeholder="Type your message..." @keyup.enter="sendMessage"
        clearable round type="textarea" />
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
