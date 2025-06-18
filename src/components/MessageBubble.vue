<script setup lang="ts">
import {
  NAvatar,
  NIcon,
  NButton,
  NFlex,
  NButtonGroup,
  NCollapse,
  NCollapseItem,
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
import { mixColours } from "../utils/colour";
import { useElementSize, useElementVisibility } from "@vueuse/core";
import { debounce } from "lodash";
import { Menu, MenuItem, PredefinedMenuItem } from "@tauri-apps/api/menu";

const chatStore = useChatStore();
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
  reasoning?: string;
  sender: MessageRole;
  timestamp: Date;
  id: string;
  over?: boolean;
  hasPrevious?: boolean;
  hasNext?: boolean;
  culling?: boolean;
  index?: number;
}>();

const emit = defineEmits<{
  (e: "resend", derive: boolean, text: string): void;
  (e: "edit"): void;
  (e: "regenerate"): void;
  (e: "previous"): void;
  (e: "next"): void;
  (e: "ready"): void;
}>();

const isStreaming = computed(() => chatStore.isStreaming);

const container = useTemplateRef<HTMLDivElement>("container");
const height = ref(0);
const rendered = ref(false);
const visible = useElementVisibility(container);

if (props.culling) {
  const size = useElementSize(container);
  watch(
    [size.height, rendered, visible],
    debounce((newVal) => {
      if (!rendered.value || isStreaming.value) return;
      height.value = Math.round(newVal[0]);
    }, 100)
  );
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
      await chatStore.deleteMessage(props.id);
      console.log("Message deleted:", props.id);
    },
  });
};

const showContextMenu = async (e: MouseEvent) => {
  e.stopPropagation();

  const menu = await Menu.new();

  const selectedText = window.getSelection()?.toString();
  if (selectedText) {
    await menu.append(
      await MenuItem.new({
        text: "Copy selected",
        action: async () => await writeText(selectedText),
      })
    );

    await menu.append(await PredefinedMenuItem.new({ item: "Separator" }));
  }

  await menu.append(
    await MenuItem.new({
      text: "Copy",
      action: () => copyMessage(),
    })
  );

  await menu.append(
    await MenuItem.new({
      text: "Delete",
      action: () => removeMessage(),
    })
  );

  await menu.append(
    await MenuItem.new({
      text: "Regenerate",
      action: () => emit("regenerate"),
    })
  );

  await menu.append(
    await MenuItem.new({
      text: "Edit",
      action: () => emit("edit"),
    })
  );

  await menu.popup();
};

const onReadyStatusChange = (ready: boolean) => {
  if (ready) emit("ready");
};

const thinkingPanelExpandedNames = ref<string[]>([]);
if (!(props.over ?? true)) thinkingPanelExpandedNames.value.push("thinking");

const footerVisible = ref(false);
</script>

<template>
  <div ref="container">
    <div
      v-if="!visible && height !== 0 && culling && !isStreaming"
      class="placeholder"
    ></div>
    <div v-else class="item-container">
      <n-flex align="start" :wrap="false" class="item-layout">
        <n-avatar class="avatar">
          <n-icon
            :component="sender === 'bot' ? Chat24Regular : Person24Regular"
          />
        </n-avatar>
        <div
          class="message-bubble"
          :class="sender"
          :id="id"
          :tabindex="10"
          @mouseenter="() => (footerVisible = true)"
          @mouseleave="() => (footerVisible = false)"
          @focusin="() => (footerVisible = true)"
          @focusout="() => (footerVisible = false)"
        >
          <div
            class="content-container"
            @contextmenu="
              (e) => {
                e.preventDefault();
                showContextMenu(e);
              }
            "
          >
            <div class="content">
              <div v-if="reasoning" class="reasoning-container">
                <n-collapse
                  arrow-placement="right"
                  display-directive="show"
                  v-model:expanded-names="thinkingPanelExpandedNames"
                >
                  <n-collapse-item title="Thinking" name="thinking">
                    <MarkdownRenderer
                      :text="reasoning"
                      :over="over"
                      v-model:ready="rendered"
                    />
                  </n-collapse-item>
                </n-collapse>
              </div>
              <MarkdownRenderer
                :text="text"
                :over="over"
                v-model:ready="rendered"
                @update:ready="onReadyStatusChange"
              />
            </div>
          </div>
          <div
            class="footer"
            :style="{ visibility: footerVisible ? 'visible' : 'hidden' }"
          >
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
                <n-button quaternary :onclick="() => emit('edit')" size="tiny">
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
  --property-will-change: v-bind('isStreaming ? "height" : "auto"');
  will-change: var(--property-will-change);
  width: 100%;
  height: v-bind('(rendered || !culling ? "fit-content" : `${height}px`)');
  animation: fade-in 0.2s v-bind("theme.cubicBezierEaseIn");
}

.item-layout {
  flex-direction: v-bind(
    'sender === "user" ? "row-reverse" : "row"'
  ) !important;
  align-items: flex-start;
  margin-bottom: 12px;
}

.avatar {
  position: sticky;
  top: 8px;
  box-shadow: v-bind("theme.boxShadow2");
}

.message-bubble {
  max-width: 80%;
  will-change: var(--property-will-change);
  width: fit-content;

  display: grid;
  grid-template-columns: auto;
  grid-template-rows: auto, auto;
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

  will-change: var(--property-will-change);
  min-width: 0;
  min-height: 0;
}

.content {
  width: fit-content;
  padding: 12px 16px;
  margin-left: 12px;
  margin-right: 12px;
  transition: all 0.2s ease;
  will-change: var(--property-will-change);

  background-color: v-bind("backgroundColor");
  border-radius: v-bind("theme.borderRadius");
  box-shadow: v-bind("theme.boxShadow2");
  border: v-bind("border");

  overflow: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.reasoning-container {
  width: 100%;
  background-color: rgba(128, 128, 128, 0.2);
  padding: 8px 12px;
  box-sizing: border-box;
  border-radius: v-bind("theme.borderRadiusSmall");
  border: 1px solid v-bind("theme.borderColor");
  box-shadow: v-bind("theme.boxShadow3");
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

.timestamp {
  font-size: 0.8em;
  margin-left: 16px;

  width: fit-content;
  font-family: monospace;
  color: v-bind("theme.textColorBase");
}
</style>
