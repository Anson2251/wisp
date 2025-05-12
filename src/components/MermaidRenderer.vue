<script setup lang="ts">
import { watch, ref } from "vue"
import { type RenderOptions } from "mermaid-isomorphic"
import { useMermaid } from "../composables/useMermaid"
import { NCode, useThemeVars } from "naive-ui";

const props = defineProps<{
  diagram: string,
  options?: RenderOptions,
}>()

const { renderDiagram } = useMermaid()
const error = ref<string | null>(null)
const diagramSvg = ref<string | null>(null)
const theme = useThemeVars()

const height = ref<number>(0)
const width = ref<number>(0)

const updateDiagram = () => {
  diagramSvg.value = ""

  renderDiagram(props.diagram, props.options)
    .then((result) => {
      if (result) {
        diagramSvg.value = result.svg
        height.value = result.height
        width.value = result.width
      } else {
        diagramSvg.value = ""
        height.value = 0
        width.value = 0
      }
    })
    .catch((err) => {
      error.value = err.message
      console.error(err)
    })
}

watch(() => props.diagram, updateDiagram, { immediate: true })
// updateDiagram()
</script>

<template>
  <div class="mermaid-renderer" :style="{
    background: theme.cardColor,
    borderColor: theme.borderColor,
    borderRadius: theme.borderRadius,
  }">
    <div v-if="error">
      <p class="error">Failed to render diagram: {{ error }}</p>
      <n-code :code="props.diagram" :inline="false" :show-line-numbers="true" />
    </div>
    <div v-else-if="!diagramSvg" class="loading">
      Rendering diagram...
    </div>
    <div v-else-if="diagramSvg && height > 0 && width > 0" class="diagram" v-html="diagramSvg" />
    <div v-else class="empty">
      No diagram to render
    </div>
  </div>
</template>

<style scoped>
.mermaid-renderer {
  width: 100%;
  height: fit-content;
  overflow: auto;

  border: 1px solid #e0e0e0;
  box-sizing: border-box;
}

.error {
  color: red;
  padding-left: 8px;
  padding-right: 8px;
  box-sizing: border-box;
  overflow-y: auto;
  max-height: 48px;
}

.loading {
  color: gray;
}

.empty {
  color: gray;
  font-style: italic;
}

.diagram {
  display: flex;
  justify-content: center;
}
</style>
