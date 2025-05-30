<script lang="ts" setup>
import { NInput, NButton, NEmpty, NIcon } from "naive-ui";
import { Chat48Regular } from "@vicons/fluent";
import MessageBubble from "./MessageBubble.vue";
import AutoScrollWrapper from "./AutoScrollWrapper.vue";
import { useChatStore } from "../stores/chat";
import { ref, provide, watch } from "vue";
import { Message, MessageRole } from "../libs/types";

import { useMermaid } from "../composables/useMermaid";
import { useVNodeRenderer } from "../composables/useMarkdown";

provide("MermaidRenderer", useMermaid());
provide("MarkdownRenderer", useVNodeRenderer());

const chatStore = useChatStore();

(window as any).chatStore = chatStore;

const autoScrollWrapper = ref<typeof AutoScrollWrapper | null>(null);

const props = defineProps({
  useBubbleCulling: {
    type: Boolean,
    default: false,
  },
  conversationId: {
    type: String,
    required: false,
  },
});

console.log(`[Chat] Message bubble culling enabled`);

const sendMessage = () => {
  if (!chatStore.userInput.trim()) return;
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
    },
  });
};

const regenerateMessage = (messageId: string, insertGuidance = false) => {
  chatStore.regenerateMessage(
    messageId,
    {
      beforeSend: () => {
        chatStore.clearUserInput();
        autoScrollWrapper.value?.scrollToBottom(false);
      },
      onReceiving: () => {
        autoScrollWrapper.value?.scrollToBottom();
      },
    },
    insertGuidance
  );
};

const resendMessage = (messageId: string, text: string, derive: boolean) => {
  if (derive) {
    chatStore.deriveMessage(messageId, text, {
      beforeSend: () => {
        chatStore.clearUserInput();
        autoScrollWrapper.value?.scrollToBottom(false);
      },
      onReceiving: () => {
        autoScrollWrapper.value?.scrollToBottom();
      },
    });
  } else {
    (async () => {
      await chatStore.updateMessage(messageId, text);
      regenerateMessage(messageId);
    })();
  }
};

const navigateToSibling = (id: string, direction: number) => {
  const index = chatStore.threadTree.getNodeDepth(id) - 1;
  chatStore.changeThreadTreeDecision(index, direction, true);
};

const loadConversationWithId = async (id?: string) => {
  if (!id) return;

  chatStore.currentConversationId = id;
  console.log("[Chat] Current conversation id: ", id);

  await chatStore.loadConversation(id);
  setTimeout(() => {
    autoScrollWrapper.value?.scrollToBottom();
  }, 500);
};

watch(
  () => props.conversationId,
  async (newId) => {
    try {
      console.log("[Chat] Watching conversation id change:", newId);
      await loadConversationWithId(newId);
      setTimeout(() => autoScrollWrapper.value?.scrollToBottom(true, false), 500);
    } catch (error) {
      console.error("[Chat] Error loading conversation:", error);
    }
  }
);
</script>

<template>
  <div v-if="chatStore.currentConversationId" class="chat-container">
    <div class="messages-container">
      <AutoScrollWrapper
        v-if="chatStore.displayedMessage.length > 0"
        ref="autoScrollWrapper"
        :auto="true"
        :smooth="true"
      >
        <div class="bubble-container">
          <MessageBubble
            v-for="(message, index) in chatStore.displayedMessage"
            :key="message.id"
            :text="message.text"
            :sender="message.sender"
            :timestamp="new Date(message.timestamp * 1000)"
            :id="message.id"
            :over="
              index === chatStore.displayedMessage.length - 1 &&
              !chatStore.isStreaming
            "
            :hasPrevious="message.hasPrevious"
            :hasNext="message.hasNext"
            :culling="useBubbleCulling"
            @previous="() => navigateToSibling(message.id, -1)"
            @next="() => navigateToSibling(message.id, 1)"
            @resend="(derive, text) => resendMessage(message.id, text, derive)"
            @regenerate="() => regenerateMessage(message.id, true)"
          />
        </div>
      </AutoScrollWrapper>
      <div v-else class="placeholder-container">
        <n-empty description="Let's chatting!">
          <template #icon>
            <n-icon :size="48">
              <Chat48Regular />
            </n-icon>
          </template>
        </n-empty>
      </div>
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
  <div v-else class="placeholder-container">
    <n-empty :show-icon="false" description="Select a conversation to start" />
  </div>
</template>

<style scoped>
.chat-container {
  display: grid;
  grid-template-rows: 1fr auto;
  height: 100%;
}

.placeholder-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
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
  padding: 8px;
}
</style>
