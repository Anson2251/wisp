<script setup lang="ts">
import { NCode, useThemeVars, NButton, NIcon, NTooltip } from "naive-ui";
import { Copy16Regular } from '@vicons/fluent'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import MermaidRenderer from "./MermaidRenderer.vue";
import { ref, computed } from "vue";

const theme = useThemeVars()

const props = defineProps<{
  code: string,
  language: string,
}>()

const copyCode = async () => {
  await writeText(props.code)
  copied.value = true
  // prevent the copied indicator changing during the animation
  const restoreCopiedIndicator = setInterval(() => {
    if (!copiedToolTipShow.value) {
      clearInterval(restoreCopiedIndicator)
      setTimeout(() => copied.value = false, 150)
    }
  }, 100)
}

const copiedToolTipShow = ref(false)
const copied = ref(false)
const copyIndicator = computed(() => {
  return copied.value ? "Copied!" : "Copy"
})
</script>

<template>
  <div>
    <mermaid-renderer v-if="props.language === 'mermaid-live'" :diagram="props.code" />
    <div v-else class="code-container" :style="{
      backgroundColor: theme.cardColor,
      borderColor: theme.borderColor,
      borderRadius: theme.borderRadius,
    }">
      <div class="language-label-container">
        <p class="language-label">{{ props.language.toLocaleUpperCase() }}</p>
        <n-tooltip v-model:show="copiedToolTipShow">
          <template #trigger>
            <n-button quaternary :onclick="copyCode" size="tiny">
              <template #icon>
                <n-icon :component="Copy16Regular" size="20" color="grey"/>
              </template>
            </n-button>
          </template>
          {{ copyIndicator }}
        </n-tooltip>
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

.code-block * {
  transition: none !important;
  animation: none !important;

  font-family: 'Consolas', 'Menlo', 'Ubuntu Mono', 'Fira Code', 'Cascadia Code', 'Courier New', Courier, monospace;
}
</style>
