<script lang="ts" setup>
import {
  NButton,
  NPopover,
  NIcon,
  NThing,
  NPerformantEllipsis,
} from "naive-ui";
import { MoreHorizontal16Regular } from "@vicons/fluent";
import { useChatStore } from "../stores/chat";
import { onMounted, ref } from "vue";
import { computedAsync } from "@vueuse/core";
import { useThemeVars } from "naive-ui";

const infoClicked = ref<boolean[]>([])

const theme = useThemeVars();
const chatStore = useChatStore();
const conversations = computedAsync(async () => {
  const result = await chatStore.listConversations()
  infoClicked.value = (new Array(result.length)).fill(false)
  return result
}, []);
const selectedId = ref<string>();

onMounted(async () => {
  if (conversations.value.length > 0) {
    selectedId.value = conversations.value[0].id;
  }
});

const emit = defineEmits<{
  (e: "select", id: string): void;
}>();

const handleSelect = (id: string) => {
  selectedId.value = id;
  emit("select", id);
};

const handleNewConversation = async () => {
  const newId = await chatStore.createConversation("New Conversation", "");
  conversations.value = await chatStore.listConversations();
  selectedId.value = newId;
  emit("select", newId);
};
</script>

<template>
  <div class="conversation-list">
    <div class="new-conversation">
      <n-button type="primary" @click="handleNewConversation">
        New Conversation
      </n-button>
    </div>
    <div
      v-for="(conv, index) in conversations"
      :class="[
        'conversation-item',
        ...(selectedId === conv.id ? ['selected'] : []),
        ...(infoClicked[index]  ? ['clicked-active'] : []),
      ]"

      :key="conv.id"
      @click="handleSelect(conv.id)"
    >
      <div class="item-title">{{ conv.name }}</div>
      <div class="item-description">
        {{ conv.description || "No description available." }}
      </div>
      <div class="item-info">
        <n-popover trigger="click" v-model:show="infoClicked[index]">
          <template #trigger>
            <n-icon :size="24">
              <more-horizontal16-regular />
            </n-icon>
          </template>
          <n-thing>
            <template #header>
              <div>{{ conv.name }}</div>
            </template>
            <template #description>
              <n-performant-ellipsis style="max-width: 16em">{{
                conv.description || "No description available."
              }}</n-performant-ellipsis>
            </template>
            <template #footer>
              <div style="font-size: 0.8em">
                <div>
                  <n-performant-ellipsis
                    style="max-width: 16em; font-family: monospace"
                  >
                    ID: {{ conv.id }}
                  </n-performant-ellipsis>
                </div>
              </div>
            </template>
          </n-thing>
        </n-popover>
      </div>
    </div>
  </div>
</template>

<style scoped>
.conversation-list {
  height: 100%;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  flex-wrap: nowrap;

  min-width: 0;
  min-height: 0;
}

.conversation-item {
  width: 100%;
  height: 4em;

  min-width: 0;
  min-height: 0;
  padding: 8px 4px 8px 12px;
  box-sizing: border-box;

  display: grid;
  grid-template-columns: auto 0;
  grid-template-rows: auto auto;
  transition: background-color,
    grid-template-columns 0.2s v-bind("theme.cubicBezierEaseOut");

  --info-icon-opacity: 0;
}

.clicked-active, .conversation-item:hover {
  grid-template-columns: auto 3em;
  background-color: v-bind("theme.hoverColor");
  transition: background-color,
    grid-template-columns 0.2s v-bind("theme.cubicBezierEaseIn");
  --info-icon-opacity: 1;
}

.item-title {
  grid-column: 1 / 2;
  grid-row: 1 / 2;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: v-bind("theme.textColor1");
  font-weight: 500;
}

.item-description {
  grid-column: 1 / 2;
  grid-row: 2 / 3;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: v-bind("theme.textColor2");
  font-size: 0.9em;
}

.item-info {
  grid-column: 2 / 3;
  grid-row: 1 / 3;

  display: grid;
  align-content: center;
  justify-content: center;
  opacity: var(--info-icon-opacity);

  transition: opacity 0.2s v-bind("theme.cubicBezierEaseInOut");
}

.conversation-list:deep(*) {
  cursor: pointer !important;
}

.selected {
  background-color: v-bind("theme.actionColor") !important;
}
</style>
