<script setup lang="ts">
import {
  NAvatar,
  NIcon,
  NButton,
  NFlex,
  NButtonGroup,
  useDialog,
  useThemeVars,
} from "naive-ui";
import {
  Chat24Regular,
  Person24Regular,
  Copy16Regular,
  Delete16Regular,
  Edit16Regular,
  ArrowClockwise16Regular,
  ChevronLeft16Regular,
  ChevronRight16Regular,
} from "@vicons/fluent";
import MarkdownRenderer from "./MarkdownRenderer.vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { MessageRole } from "../libs/types";
import { useChatStore } from "../stores/chat";
import { ref, computed, useTemplateRef, watch } from "vue";
import MessageBubbleEditor from "./MessageBubbleEditor.vue";
import { mixColours } from "../utils/colour";
import { useElementSize, useElementVisibility } from "@vueuse/core";

const { deleteMessage } = useChatStore();
const dialog = useDialog();
const theme = useThemeVars();

const borderColor = computed(() =>
  props.sender === MessageRole.User ? "transparent" : theme.value.borderColor
);

const backgroundColor = computed(() =>
  props.sender === MessageRole.User
    ? mixColours(theme.value.primaryColor, theme.value.baseColor, 0.3)
    : theme.value.cardColor
);

const border = computed(() => `1px solid ${borderColor.value}`);

const props = defineProps<{
  text: string;
  sender: MessageRole;
  timestamp: Date;
  id: string;
  over?: boolean;
  hasPrevious?: boolean;
  hasNext?: boolean;
  culling?: boolean;
}>();

const emit = defineEmits<{
  (e: "resend", derive: boolean, text: string): void;
  (e: "regenerate"): void;
  (e: "previous"): void;
  (e: "next"): void;
}>();

const container = useTemplateRef<HTMLDivElement>("container");
const height = ref(0);
const rendered = ref(false);
const visible = useElementVisibility(container);

if (props.culling) {
  const size = useElementSize(container);

  watch([size.height, rendered, visible], (newVal) => {
    if (!visible.value || !rendered.value) return;
    height.value = Math.round(newVal[0]);
  });
}

const copyMessage = async () => {
  await writeText(props.text);
  const bubble = document.querySelector(`.message-bubble[id="${props.id}"]`);
  if (bubble) {
    bubble.classList.add("copied");
    setTimeout(() => bubble.classList.remove("copied"), 500);
  }
};

const removeMessage = () => {
  dialog.warning({
    title: "Delete Message",
    content: "Are you sure you want to delete this message?",
    positiveText: "Delete",
    negativeText: "Cancel",
    onPositiveClick: async () => {
      await deleteMessage(props.id);
      console.log("Message deleted:", props.id);
    },
  });
};

const showEditorModal = ref(false);
const editMessage = () => {
  showEditorModal.value = true;
};
</script>

<template>
  <div ref="container">
    <div v-if="!visible && height !== 0 && culling" class="placeholder"></div>
    <div v-else class="item-container">
      <n-flex
        align="start"
        :wrap="false"
        :style="{
          flexDirection: sender === 'user' ? 'row-reverse' : 'row',
          alignItems: 'flex-start',
          marginBottom: '12px',
        }"
      >
        <n-avatar style="position: sticky; top: 8px">
          <n-icon
            :component="sender === 'bot' ? Chat24Regular : Person24Regular"
          />
        </n-avatar>
        <div class="message-bubble" :class="sender" :id="id">
          <div class="content-container">
            <div class="content">
              <MarkdownRenderer
                :text="text"
                :over="over"
                v-model:ready="rendered"
              />
            </div>
          </div>
          <div class="footer">
            <n-flex :wrap="false" align="center">
              <n-button-group class="button-group">
                <n-button quaternary :onclick="copyMessage" size="tiny">
                  <template #icon>
                    <n-icon :component="Copy16Regular" :size="16" />
                  </template>
                </n-button>
                <n-button
                  quaternary
                  :onclick="removeMessage"
                  type="error"
                  size="tiny"
                >
                  <template #icon>
                    <n-icon :component="Delete16Regular" :size="18" />
                  </template>
                </n-button>
                <n-button
                  quaternary
                  @click="emit('regenerate')"
                  size="tiny"
                  v-if="sender === 'bot'"
                >
                  <template #icon>
                    <n-icon :component="ArrowClockwise16Regular" :size="16" />
                  </template>
                </n-button>
                <n-button quaternary :onclick="editMessage" size="tiny">
                  <template #icon>
                    <n-icon :component="Edit16Regular" :size="16" />
                  </template>
                </n-button>
              </n-button-group>
              <n-button-group class="nav-group" v-if="hasPrevious || hasNext">
                <n-button
                  quaternary
                  @click="emit('previous')"
                  size="tiny"
                  :disabled="!hasPrevious"
                >
                  <template #icon>
                    <n-icon :component="ChevronLeft16Regular" :size="16" />
                  </template>
                </n-button>
                <n-button
                  quaternary
                  @click="emit('next')"
                  size="tiny"
                  :disabled="!hasNext"
                >
                  <template #icon>
                    <n-icon :component="ChevronRight16Regular" :size="16" />
                  </template>
                </n-button>
              </n-button-group>
            </n-flex>
            <span class="timestamp">{{ timestamp.toLocaleTimeString() }}</span>
          </div>
        </div>
      </n-flex>
      <MessageBubbleEditor
        v-model:show="showEditorModal"
        :id="id"
        @resend="(derive, text) => emit('resend', derive, text)"
      />
    </div>
  </div>
</template>

<style scoped>
@keyframes fade-in {
  from {
    /* transform: scale(0.95); */
    opacity: 0;
  }
  to {
    /* transform: scale(1); */
    opacity: 1;
  }
}

.item-container {
  transform-origin: bottom 30%;
  width: 100%;
  height: v-bind('(rendered || !culling ? "fit-content" : `${height}px`)');
  animation: fade-in 0.2s v-bind("theme.cubicBezierEaseIn");
}

.message-bubble {
  max-width: 80%;
  width: fit-content;

  display: grid;
  grid-template-columns: auto;
  grid-template-rows: auto, auto;

  --footer-buttons-opacity: 0;
}

.message-bubble:hover {
  --footer-buttons-opacity: 1;
}

.message-bubble.user {
  color: white;
  margin-left: auto;
}

.message-bubble.bot {
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

.content-container {
  grid-area: 1 / 1 / 2 / 2;
  display: flex;
  justify-content: v-bind('sender === "bot" ? "flex-start" : "flex-end"');

  min-width: 0;
  min-height: 0;
}

.content {
  width: fit-content;
  padding: 12px 16px;
  margin-left: 12px;
  margin-right: 12px;
  transition: all 0.2s ease;

  background-color: v-bind("backgroundColor");
  border-radius: v-bind("theme.borderRadius");
  box-shadow: v-bind("theme.boxShadow2");
  border: v-bind("border");

  overflow: auto;
}

.footer {
  padding: 8px 16px 0 16px;
  width: 100%;
  box-sizing: border-box;
  min-width: fit-content;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.placeholder {
  pointer-events: none;
  user-select: none;

  width: 100%;
  height: v-bind("`${height}px`");
}

/** I don't know why the timestamp doesn't disappear in Safari once it appears */
.footer:deep(*) {
  /* Work around for Safari bug */
  opacity: var(--footer-buttons-opacity);
  transition: opacity 0.2s v-bind("theme.cubicBezierEaseInOut");
}

.timestamp {
  font-size: 0.8em;
  margin-left: 16px;

  width: fit-content;
  font-family: monospace;
  /* opacity: var(--footer-buttons-opacity); */
  color: v-bind("theme.textColorBase");
}
</style>
