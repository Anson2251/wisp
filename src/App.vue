<script setup lang="ts">
import { NSplit } from "naive-ui";
import Chat from "./components/Chat.vue";
import ConversationList from "./components/ConversationList.vue";
import {
  NDialogProvider,
  NConfigProvider,
  NModalProvider,
  NMessageProvider,
  NGlobalStyle,
  useOsTheme,
  darkTheme,
  lightTheme,
} from "naive-ui";
import katex from "katex";
import hljs from "highlight.js/lib/core";
import { provide, computed, ref } from "vue";
import { addAlpha } from "./utils/colour";

import javascript from "highlight.js/lib/languages/javascript";
import typescript from "highlight.js/lib/languages/typescript";
import html from "highlight.js/lib/languages/xml";
import css from "highlight.js/lib/languages/css";
import python from "highlight.js/lib/languages/python";
import java from "highlight.js/lib/languages/java";
import cpp from "highlight.js/lib/languages/cpp";
import csharp from "highlight.js/lib/languages/csharp";
import php from "highlight.js/lib/languages/php";
import ruby from "highlight.js/lib/languages/ruby";
import go from "highlight.js/lib/languages/go";
import rust from "highlight.js/lib/languages/rust";
import swift from "highlight.js/lib/languages/swift";
import sql from "highlight.js/lib/languages/sql";
import bash from "highlight.js/lib/languages/bash";
import json from "highlight.js/lib/languages/json";
import yaml from "highlight.js/lib/languages/yaml";
import markdown from "highlight.js/lib/languages/markdown";
import prolog from "highlight.js/lib/languages/prolog";
import shell from "highlight.js/lib/languages/shell";
import dockerfile from "highlight.js/lib/languages/dockerfile";
import kotlin from "highlight.js/lib/languages/kotlin";
import perl from "highlight.js/lib/languages/perl";
import lua from "highlight.js/lib/languages/lua";
import r from "highlight.js/lib/languages/r";

import { useOpenAI } from "./composables/useOpenAI";
const openai = useOpenAI();

provide("OpenAI", openai);

hljs.registerLanguage("javascript", javascript);
hljs.registerLanguage("typescript", typescript);
hljs.registerLanguage("html", html);
hljs.registerLanguage("css", css);
hljs.registerLanguage("python", python);
hljs.registerLanguage("java", java);
hljs.registerLanguage("cpp", cpp);
hljs.registerLanguage("csharp", csharp);
hljs.registerLanguage("php", php);
hljs.registerLanguage("ruby", ruby);
hljs.registerLanguage("go", go);
hljs.registerLanguage("rust", rust);
hljs.registerLanguage("swift", swift);
hljs.registerLanguage("sql", sql);
hljs.registerLanguage("bash", bash);
hljs.registerLanguage("json", json);
hljs.registerLanguage("yaml", yaml);
hljs.registerLanguage("markdown", markdown);
hljs.registerLanguage("prolog", prolog);
hljs.registerLanguage("shell", shell);
hljs.registerLanguage("sh", shell);

hljs.registerLanguage("dockerfile", dockerfile);
hljs.registerLanguage("kotlin", kotlin);
hljs.registerLanguage("perl", perl);
hljs.registerLanguage("lua", lua);
hljs.registerLanguage("r", r);

const osThemeRef = useOsTheme();
const isDark = computed(() => osThemeRef.value === "dark");
const theme = computed(() => (isDark.value ? darkTheme : lightTheme));

const themeOverrides = computed(() => ({
  common: {
    bodyColor: addAlpha(theme.value.common.bodyColor, isDark.value ? 0.7 : 0.6),
  },
}));

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
    :theme-overrides="themeOverrides"
  >
    <n-dialog-provider>
      <n-modal-provider>
        <n-message-provider>
          <n-split
            direction="horizontal"
            style="height: 100vh"
            :max="'400px'"
            :min="'0px'"
            :default-size="'320px'"
          >
            <template #1>
              <div style="height: 100%">
                <ConversationList @select="handleConversationSelect" />
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
    <n-global-style />
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
