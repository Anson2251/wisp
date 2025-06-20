<script lang="ts" setup>
import {
  NInput,
  NButton,
  NEmpty,
  NIcon,
  NSelect,
  useThemeVars,
  useMessage,
  type SelectOption,
} from "naive-ui";
import { Chat48Regular, Send20Regular } from "@vicons/fluent";
import MessageBubble from "./MessageBubble.vue";
import AutoScrollWrapper from "./AutoScrollWrapper.vue";
import { ref, inject, watch, onMounted, computed } from "vue";
import { Message, MessageRole } from "../libs/types";
import { useProviderStore } from "../stores/provider";
import MessageBubbleEditor from "./MessageBubbleEditor.vue";
import { useChatStore } from "../stores/chat";

const theme = useThemeVars();
const notificationMessage = useMessage();

const chatStore = inject("ChatStore") as ReturnType<typeof useChatStore>;
const providerStore = inject("ProviderStore") as ReturnType<
  typeof useProviderStore
>;

const providerOptions = computed<SelectOption[]>(() =>
  providerStore.providers.map((p) => ({
    label: p.display_name,
    value: p.name,
  }))
);

const modelOptions = computed<SelectOption[]>(
  () =>
    chatStore.chosenProvider?.models
      .filter((m) => m.model_info.type === "text_generation")
      .map((m) => ({
        label: m.metadata.display_name,
        value: m.metadata.name,
      })) || []
);

const chosenProviderId = ref<string | null>(null);
watch(chosenProviderId, (newId) => {
  if (newId) {
    chatStore.chosenProvider =
      providerStore.providers.find((p) => p.name === newId) ?? null;
  } else {
    chatStore.chosenProvider = null;
  }
});

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
  chatStore
    .sendMessage(userMessage, {
      beforeSend: () => {
        chatStore.clearUserInput();
        autoScrollWrapper.value?.scrollToBottom(false);
      },
      onReceiving: () => {
        autoScrollWrapper.value?.scrollToBottom(false);
      },
      onFinish: () => {
        setTimeout(() => autoScrollWrapper.value?.scrollToBottom(false), 1000);
      },
    })
    .catch((e) => notificationMessage.error(e));
};

const regenerateMessage = (messageId: string, insertGuidance = false) => {
  chatStore
    .regenerateMessage(
      messageId,
      {
        beforeSend: () => {
          chatStore.clearUserInput();
          autoScrollWrapper.value?.scrollToBottom(false);
        },
        onReceiving: () => {
          autoScrollWrapper.value?.scrollToBottom(false);
        },
        onFinish: () => {
          setTimeout(
            () => autoScrollWrapper.value?.scrollToBottom(false),
            1000
          );
        },
      },
      insertGuidance
    )
    .catch((e) => notificationMessage.error(e));
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
  bubbleReadyCount.value = index + 1;
  console.time("[Chat] Message list loaded");
  chatStore.changeThreadTreeDecision(index, direction, true);
};

const allMessageBubbleReady = () => {
  console.timeEnd("[Chat] Message list loaded");
  setTimeout(() => autoScrollWrapper.value?.scrollToBottom(true, false), 300);
};
const bubbleReadyCount = ref(0);
watch(bubbleReadyCount, () => {
  if (bubbleReadyCount.value === chatStore.displayedMessage.length) {
    allMessageBubbleReady();
  }
});

const loadConversationWithId = async (id?: string) => {
  if (!id) return;

  console.time("[Chat] Message list loaded");
  bubbleReadyCount.value = 0;

  chatStore.currentConversationId = id;
  console.log("[Chat] Current conversation id: ", id);

  await chatStore.loadConversation(id);
};

const showEditorModal = ref(false);
const messageEditingId = ref<string | null>(null);

const showEditor = (messageId: string) => {
  messageEditingId.value = messageId;
  showEditorModal.value = true;
};

onMounted(() => {
  watch(
    () => props.conversationId,
    async (newId) => {
      try {
        console.log("[Chat] Watching conversation id change:", newId);
        await loadConversationWithId(newId);
      } catch (error) {
        console.error("[Chat] Error loading conversation:", error);
      }
    }
  );
});
</script>

<template>
  <div style="height: 100%; width: 100%">
    <div v-if="chatStore.currentConversationId" class="chat-container">
      <div class="messages-container">
        <auto-scroll-wrapper
          v-if="chatStore.displayedMessage.length > 0"
          ref="autoScrollWrapper"
          :auto="true"
          :smooth="true"
        >
          <div class="bubble-container">
            <message-bubble
              v-for="(message, index) in chatStore.displayedMessage"
              :key="message.id"
              :text="message.text"
              :reasoning="message.reasoning"
              :sender="message.sender"
              :timestamp="new Date(message.timestamp * 1000)"
              :id="message.id"
              :over="
                !(
                  index === chatStore.displayedMessage.length - 1 &&
                  chatStore.isStreaming
                )
              "
              :index="index"
              :hasPrevious="message.hasPrevious"
              :hasNext="message.hasNext"
              :culling="useBubbleCulling"
              @previous="() => navigateToSibling(message.id, -1)"
              @next="() => navigateToSibling(message.id, 1)"
              @edit="() => showEditor(message.id)"
              @regenerate="() => regenerateMessage(message.id, true)"
              @ready="() => (bubbleReadyCount += 1)"
            />
          </div>
        </auto-scroll-wrapper>
        <div v-else class="placeholder-container">
          <n-empty description="Let's chatting!">
            <template #icon>
              <n-icon :size="48">
                <Chat48Regular />
              </n-icon>
            </template>
          </n-empty>
        </div>
        <div class="messages-container-shadow shadow-top"></div>
        <div class="messages-container-shadow shadow-bottom"></div>
      </div>

      <div class="input-container">
        <n-space vertical>
          <n-space justify="space-between" :wrap-items="false">
            <n-space :wrap-items="false" align="center" size="small">
              <n-select
                v-model:value="chosenProviderId"
                :options="providerOptions"
                placeholder="Select provider"
                :consistent-menu-width="false"
                clearable
                filterable
                style="width: 8em;"
              />
              <span style="font-size: 1.5em;">/</span>
              <n-select
                v-model:value="chatStore.chosenModel"
                :options="modelOptions"
                placeholder="Select model"
                :consistent-menu-width="false"
                clearable
                filterable
                :disabled="!chatStore.chosenProvider"
                style="min-width: 12em;"
              />
            </n-space>
            <n-button
              type="primary"
              @click="sendMessage"
              circle
              :disabled="!chatStore.userInput || !chatStore.chosenModel"
              ><template #icon>
                <n-icon :size="20">
                  <Send20Regular />
                </n-icon>
              </template>
            </n-button>
          </n-space>
          <n-input
            v-model:value="chatStore.userInput"
            placeholder="Type your message..."
            @keyup.enter="sendMessage"
            clearable
            round
            type="textarea"
          />
        </n-space>
      </div>
    </div>
    <div v-else class="placeholder-container">
      <n-empty
        :show-icon="false"
        description="Select a conversation to start"
      />
    </div>
    <message-bubble-editor
      v-model:show="showEditorModal"
      :id="messageEditingId ?? ''"
      @resend="
        (derive: boolean, text: string) => {
          if (!messageEditingId) return;
          resendMessage(messageEditingId, text, derive);
          messageEditingId = '';
        }
      "
    />
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
  position: relative;
}

.messages-container-shadow {
  width: 100%;
  height: 8px;
  position: absolute;
  --from-colour: v-bind("theme.cardColor");
}

.shadow-bottom {
  background: linear-gradient(to bottom, transparent, var(--from-colour));
  bottom: 0;
}

.shadow-top {
  background: linear-gradient(to top, transparent, var(--from-colour));
  top: 0;
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
