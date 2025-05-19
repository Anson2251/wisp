<script lang="ts" setup>
import { NInput, NButton } from "naive-ui";
import MessageBubble from "./MessageBubble.vue";
import AutoScrollWrapper from "./AutoScrollWrapper.vue";
import { useChatStore } from "../stores/chat";
import { onMounted, ref, inject, provide } from "vue";
import debounce from "lodash/debounce";
import { Message, MessageRole } from "../libs/types";

import { useOpenAI } from "../composables/useOpenAI";
import { useMermaid } from "../composables/useMermaid";
import { useVNodeRenderer } from "../composables/useMarkdown";

provide("MermaidRenderer", useMermaid());
provide("MarkdownRenderer", useVNodeRenderer());

const chatStore = useChatStore();
const { streamResponse, isStreaming } = inject("OpenAI") as ReturnType<
  typeof useOpenAI
>;
const autoScrollWrapper = ref();

async function sendMessage() {
  if (!chatStore.userInput.trim()) return;

  const userMessage: Omit<Message, "id"> = {
    text: chatStore.userInput,
    sender: MessageRole.User,
    timestamp: Math.round(new Date().getTime() / 1000),
  };
  const userMessageId = await chatStore.addMessage(userMessage);
  chatStore.clearUserInput();

  const botMessage: Omit<Message, "id"> = {
    text: "Generating...",
    sender: MessageRole.Assistant,
    timestamp: Math.round(new Date().getTime() / 1000),
  };

  const botMessageId = await chatStore.addMessage(botMessage, userMessageId);

  const updateMessage = (text: string) => {
    const message = chatStore.messages.get(botMessageId);
    if (message) chatStore.messages.set(botMessageId, { ...message, text });
  };
  const updateBubbleText = debounce(updateMessage, 10);
  autoScrollWrapper.value?.scrollToBottom(false);

  let responseText = "";
  await streamResponse(
    chatStore.displayedMessage.map((msg) => ({
      role: msg.sender === "user" ? "user" : "assistant",
      content: msg.text,
    })),
    (chunk) => {
      responseText += chunk;
      updateBubbleText(responseText);
      autoScrollWrapper.value?.scrollToBottom();
    },
    () => {
      // const messageIndex = chatStore.messages.findIndex(
      //   (m) => m.id === botMessageId,
      // );
      // if (messageIndex < 0) return;
      // const message = chatStore.messages[messageIndex];
      // message.text = responseText;
      // chatStore.updateMessage(botMessageId, responseText);
    },
    true,
  );
}

const navigateToSibling = (index: number, direction: number) => {
  if (index < 0 || index >= chatStore.displayedMessage.length) return;

  chatStore.threadTreeDecisions[index] += direction;
};

onMounted(async () => {
  try {
    const conversations = await chatStore.listConversations();
    let conversationId = "";

    if (conversations.length == 0) {
      conversationId = await chatStore.createConversation(
        "Untitled Conversation",
        "",
      );
    } else conversationId = conversations[0].id;

    chatStore.currentConversationId = conversationId;
    console.log("Current conversation id: ", conversationId);

    chatStore.loadConversation(conversationId).then(() => {
      setTimeout(() => autoScrollWrapper.value?.scrollToBottom(false, false), 100);
    });
  } catch (error) {
    console.error("Error loading messages:", error);
  }
});
</script>

<template>
  <div class="chat-container">
    <div class="messages-container">
      <AutoScrollWrapper ref="autoScrollWrapper" :auto="true" :smooth="true">
        <div class="bubble-container">
          <MessageBubble
            v-for="(message, index) in chatStore.displayedMessage"
            :key="message.id"
            :text="message.text"
            :sender="message.sender"
            :timestamp="new Date(message.timestamp * 1000)"
            :id="message.id"
            :over="index === chatStore.displayedMessage.length - 1 && isStreaming"
            :hasPrevious="message.hasPrevious"
            :hasNext="message.hasNext"
            @previous="() => navigateToSibling(index, -1)"
            @next="() => navigateToSibling(index, 1)"
          />
        </div>
      </AutoScrollWrapper>
    </div>

    <div class="input-container">
      <n-input
        v-model:value="chatStore.userInput"
        placeholder="Type your message..."
        @keyup.enter="sendMessage"
        clearable
        round
        type="textarea"
      />
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
