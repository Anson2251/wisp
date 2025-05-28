<script setup lang="ts">
import { NCode, useThemeVars, NButton, NIcon } from "naive-ui";
import CopyIndicator from "./CopyIndicator.vue";
import { Copy16Regular } from '@vicons/fluent'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import MermaidRenderer from "./MermaidRenderer.vue";

const theme = useThemeVars()
const emits = defineEmits<{
  (e: 'ready', success: boolean): void
}>()

const props = defineProps<{
  code: string,
  language: string,
}>()
</script>

<template>
  <div>
    <div v-if="props.language === 'mermaid-live'" style="max-width: 100%; overflow: auto;">
      <mermaid-renderer :diagram="props.code" @ready="emits('ready', $event)" />
    </div>
    <div v-else class="code-container" :style="{
      backgroundColor: theme.cardColor,
      borderColor: theme.borderColor,
      borderRadius: theme.borderRadius,
      '--border-radius': theme.borderRadiusSmall,
    }">
      <div class="language-label-container">
        <p class="language-label">{{ props.language.toLocaleUpperCase() }}</p>
        <copy-indicator :text-to-copy="'Copy'" :text-copied="'Copied'" :on-copy="async() => await writeText(props.code)">
          <template #trigger>
            <n-button quaternary size="tiny">
              <template #icon>
                <n-icon :component="Copy16Regular" size="20" color="grey"/>
              </template>
            </n-button>
          </template>
        </copy-indicator>
      </div>
      <n-code class="code-block" :code="props.code"
        :language="props.language === 'mermaid-generating' ? 'mermaid' : props.language" :show-line-numbers="true" />
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
  margin-top: 4px;
  margin-bottom: 4px;
  border-radius: 8px;
  border: 1px solid #e0e0e0;

  overflow: auto;
}

.language-label-container {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  gap: 4px;
  grid-area: 1 / 1 / 2 / 2;
}

.language-label {
  font-size: 14px;
  font-weight: 500;
  color: grey;
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

.code-block {
  transition: none !important;
  animation: none !important;
  padding: 8px 12px 8px 12px ;
  box-sizing: border-box;
  border-radius: var(--border-radius);

  background-color: rgba(128, 128,128, 0.1);
}

.code-block:deep(*) {
  transition: none !important;
  animation: none !important;
  font-family: 'Consolas', 'Menlo', 'Ubuntu Mono', 'Fira Code', 'Cascadia Code', 'Courier New', Courier, monospace;
}

.code-block:deep(.__code__) {
  overflow: auto;
}
</style>
