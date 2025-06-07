<script setup lang="ts">
import { NSplit } from "naive-ui";
import Chat from "./components/Chat.vue";
import ConversationList from "./components/ConversationList.vue";
import {
  NDialogProvider,
  NConfigProvider,
  NModalProvider,
  NMessageProvider,
  useOsTheme,
  darkTheme,
  lightTheme,
} from "naive-ui";
import katex from "katex";
import { provide, computed, ref } from "vue";
import useHighlightjs from "./composables/useHighlightjs";
const hljs = useHighlightjs();

import { useOpenAI } from "./composables/useOpenAI";
const openai = useOpenAI();

provide("OpenAI", openai);

const osThemeRef = useOsTheme();
const isDark = computed(() => osThemeRef.value === "dark");
const theme = computed(() => (isDark.value ? darkTheme : lightTheme));

const selectedConversationId = ref<string>();
const handleConversationSelect = (id: string) => {
  selectedConversationId.value = id;
};
</script>

<template>
  <n-config-provider
    :katex="(katex as any)"
    :hljs="hljs"
    :theme="theme"
  >
    <n-dialog-provider>
      <n-modal-provider>
        <n-message-provider>
          <n-split
            direction="horizontal"
            style="height: 100vh"
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
                    backgroundColor: theme.common.cardColor,
                  }"
                />
              </div>
            </template>
          </n-split>
        </n-message-provider>
      </n-modal-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>
