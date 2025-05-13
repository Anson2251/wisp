<script setup lang="ts">
import { NCode, useThemeVars } from "naive-ui";
import MermaidRenderer from "./MermaidRenderer.vue";

const theme = useThemeVars()

const props = defineProps<{
  code: string,
  language: string,
}>()
</script>

<template>
  <div>
    <mermaid-renderer v-if="props.language === 'mermaid-live'"  :diagram="props.code" />
    <div v-else class="code-container" :style="{
      backgroundColor: theme.cardColor,
      borderColor: theme.borderColor,
      borderRadius: theme.borderRadius,
    }">
      <p class="language-label">{{ props.language.toLocaleUpperCase()  }}</p>
      <n-code class="code-block" :code="props.code" :language="props.language === 'mermaid-generating' ? 'mermaid' : props.language" :show-line-numbers="true" />
    </div>
  </div>
</template>

<style lang="css" scoped>
.code-container {
  display: grid;
  height: fit-content;
  width: 100%;
  gap: 8px;

  box-sizing: border-box;
  padding: 8px;
  border-radius: 8px;
  border: 1px solid #e0e0e0;

  overflow: auto;
}

.language-label {
  font-size: 14px;
  font-weight: 500;
  color: grey;

  grid-area: 1 / 1 / 2 / 2;

  padding: 0;
  margin: 0;
  margin-left: 4px;
}

.code-block {
  grid-area: 2 / 1 / 3 / 2;
  padding: 0;
  margin: 0;

  width: 100%;
  height: 100%;

  overflow: auto;
}

.code-block * {
  transition: none !important;
  animation: none !important;

  font-family: 'Consolas', 'Menlo', 'Ubuntu Mono', 'Fira Code', 'Cascadia Code', 'Courier New', Courier, monospace;
}
</style>
