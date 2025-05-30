<script lang="ts" setup>
import { NButton } from "naive-ui";
import { useChatStore } from "../stores/chat";
import { onMounted, ref } from "vue";
import { useThemeVars } from "naive-ui";
import { Menu, MenuItem } from "@tauri-apps/api/menu";
import ConversationInfoDialog from "./ConversationInfoDialog.vue";
import { useMessage } from "naive-ui";

const theme = useThemeVars();
const message = useMessage();
const chatStore = useChatStore();
const conversations = chatStore.conversations
const selectedId = ref<string>();

chatStore.listConversations();

// Info dialog state
const showInfoDialog = ref(false);
const activeConversation = ref<any>(null);

onMounted(async () => {
  if (conversations.length > 0) {
    selectedId.value = conversations[0].id;
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
  selectedId.value = newId;
  emit("select", newId);
};

const handleDeleteConversation = async (id: string) => {
  try {
  await chatStore.deleteConversation(id);
  if (selectedId.value === id && conversations.length > 0) {
    selectedId.value = conversations[0].id;
    emit("select", selectedId.value);
  } else if (conversations.length === 0) {
    selectedId.value = undefined;
  }
}
catch (e) {
  message.error(e as string)
}
};

const showContextMenu = async (e: MouseEvent, conversation: any) => {
  e.stopPropagation();

  const menu = await Menu.new();

  await menu.append(
    await MenuItem.new({
      text: "Delete",
      action: () => handleDeleteConversation(conversation.id),
    })
  );

  await menu.append(
    await MenuItem.new({
      text: "Info",
      action: () => {
        activeConversation.value = conversation;
        showInfoDialog.value = true;
      },
    })
  );

  await menu.popup();
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
      v-for="conv in chatStore.conversations"
      :class="[
        'conversation-item',
        ...(selectedId === conv.id ? ['selected'] : []),
      ]"
      :key="conv.id"
      @click="handleSelect(conv.id)"
      @contextmenu="
        (e) => {
          e.preventDefault();
          showContextMenu(e, conv);
        }
      "
    >
      <div class="item-title">{{ conv.name }}</div>
      <div class="item-description">
        {{ conv.description || "No description available." }}
      </div>
    </div>

    <conversation-info-dialog
      v-if="showInfoDialog && activeConversation"
      :conversation="activeConversation"
      :onClose="
        () => {
          showInfoDialog = false;
          activeConversation = null;
        }
      "
    />
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
  grid-template-columns: auto;
  grid-template-rows: auto auto;
  transition: background-color,
    grid-template-columns 0.2s v-bind("theme.cubicBezierEaseOut");

  --info-icon-opacity: 0;
}

.conversation-item:hover {
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
  cursor: pointer;
}

.selected {
  background-color: v-bind("theme.actionColor") !important;
}
</style>
