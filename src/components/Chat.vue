<script lang="ts" setup>
import { NInput, NButton } from "naive-ui";
import MessageBubble from "./MessageBubble.vue";
import AutoScrollWrapper from "./AutoScrollWrapper.vue";
import { useChatStore } from "../stores/chat";
import { onMounted, ref, provide, watch } from "vue";
import { Message, MessageRole } from "../libs/types";

import { useMermaid } from "../composables/useMermaid";
import { useVNodeRenderer } from "../composables/useMarkdown";

provide("MermaidRenderer", useMermaid());
provide("MarkdownRenderer", useVNodeRenderer());

const chatStore = useChatStore();

const autoScrollWrapper = ref<typeof AutoScrollWrapper | null>(null);

const props = defineProps({
  useBubbleCulling: {
    type: Boolean,
    default: false
  }
})

watch(() => chatStore.threadTreeDecisions, () => {
  setTimeout(() => autoScrollWrapper.value?.scrollToBottom(true, true), 200);
});

const sendMessage = () => {
  if (!chatStore.userInput.trim()) return
  const userMessage: Omit<Message, "id"> = {
    text: chatStore.userInput,
    sender: MessageRole.User,
    timestamp: Math.round(new Date().getTime() / 1000),
  };
  chatStore.sendMessage(userMessage, {
    beforeSend: () => {
      chatStore.clearUserInput();
      autoScrollWrapper.value?.scrollToBottom(false);
    },
    onReceiving: () => {
      autoScrollWrapper.value?.scrollToBottom();
    }
  })
}

const regenerateMessage = (messageId: string, insertGuidance = false) => {
  chatStore.regenerateMessage(messageId, {
    beforeSend: () => {
      chatStore.clearUserInput();
      autoScrollWrapper.value?.scrollToBottom(false);
    },
    onReceiving: () => {
      autoScrollWrapper.value?.scrollToBottom();
    }
  }, insertGuidance);
}

const resendMessage = (messageId: string, text: string, derive: boolean) => {
  if (derive) {
    chatStore.deriveMessage(messageId, text, {
      beforeSend: () => {
        chatStore.clearUserInput();
        autoScrollWrapper.value?.scrollToBottom(false);
      },
      onReceiving: () => {
        autoScrollWrapper.value?.scrollToBottom();
      }
    });
  } else {
    (async () => {
      await chatStore.updateMessage(messageId, text)
      regenerateMessage(messageId)
    })();
  }
}

const navigateToSibling = (id: string, direction: number) => {
  const index = chatStore.threadTree.getNodeDepth(id) - 1;
  chatStore.changeThreadTreeDecision(index, direction, true);
};

onMounted(async () => {
  if (props.useBubbleCulling) console.log(`[Chat] Message bubble culling enabled`)
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
    console.log("[Chat] Current conversation id: ", conversationId);

    await chatStore.loadConversation(conversationId)
    setTimeout(() => {
      autoScrollWrapper.value?.scrollToBottom();
    }, 500);
  } catch (error) {
    console.error("[Chat] Error loading messages:", error);
  }
});
</script>

<template>
  <div class="chat-container">
    <div class="messages-container">
      <AutoScrollWrapper
        ref="autoScrollWrapper"
        :auto="true"
        :smooth="true">
        <div class="bubble-container">
          <MessageBubble
            v-for="(message, index) in chatStore.displayedMessage"
            :key="message.id"
            :text="message.text"
            :sender="message.sender"
            :timestamp="new Date(message.timestamp * 1000)"
            :id="message.id"
            :over="index === chatStore.displayedMessage.length - 1 && !chatStore.isStreaming"
            :hasPrevious="message.hasPrevious"
            :hasNext="message.hasNext"
            :culling="useBubbleCulling"
            @previous="() => navigateToSibling(message.id, -1)"
            @next="() => navigateToSibling(message.id, 1)"
            @resend="(derive, text) => resendMessage(message.id, text, derive)"
            @regenerate="() => regenerateMessage(message.id, true)" />
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
