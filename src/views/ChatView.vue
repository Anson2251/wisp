<script setup lang="ts">
import { ref } from "vue";
import { NSplit, useThemeVars } from "naive-ui";
import ConversationList from "../components/ConversationList.vue";
import Chat from "../components/Chat.vue";

const selectedConversationId = ref<string>();
const handleConversationSelect = (id: string) => {
  selectedConversationId.value = id;
};

const theme = useThemeVars();
</script>

<template>
  <n-split
    direction="horizontal"
    :max="'240px'"
    :min="'128px'"
    :default-size="'160px'"
  >
    <template #1>
      <div style="height: 100%">
        <conversation-list @select="handleConversationSelect" />
      </div>
    </template>
    <template #2>
      <div style="height: 100%">
        <Chat
          :use-bubble-culling="true"
          :conversation-id="selectedConversationId"
          :style="{
            backgroundColor: theme.cardColor,
          }"
        />
      </div>
    </template>
  </n-split>
</template>
