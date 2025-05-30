<script setup lang="ts">
import { watch, ref, computed, inject } from "vue"
import { useMermaid } from "../composables/useMermaid"
import { NCode, NImage, useThemeVars, GlobalThemeOverrides, useOsTheme } from "naive-ui";
import { encodeBase64 } from "../utils/common";
import { MermaidConfig } from "mermaid";

const props = defineProps<{
  diagram: string,
  options?: MermaidConfig,
}>()

const { renderDiagram } = inject('MermaidRenderer') as ReturnType<typeof useMermaid>

const imageGroupThemeOverrides = computed(() => {
  const { popoverColor, boxShadow2, textColor2, borderRadius }
    = useThemeVars().value
  const themeOverrides: NonNullable<GlobalThemeOverrides['Image']> = {
    toolbarColor: popoverColor,
    toolbarBoxShadow: boxShadow2,
    toolbarIconColor: textColor2,
    toolbarBorderRadius: borderRadius
  }
  return themeOverrides
})

const error = ref<string | null>(null)
const diagramSvg = ref<string | null>(null)
const diagramSrc = computed(() => {
  try {
    if (diagramSvg.value) {

      return `data:image/svg+xml;base64,${encodeBase64(diagramSvg.value)}`;
    }
    return "";
  }
  catch (e) {
    console.error(e)
    error.value = "Failed to convert diagram to base64: " + e
    return ""
  }
})
const theme = useThemeVars()

const emits = defineEmits<{
  (e: 'ready', success: boolean): void
}>()

const height = ref<number>(0)
const width = ref<number>(0)

const osThemeRef = useOsTheme()

const updateDiagram = () => {
  diagramSvg.value = ""

  renderDiagram(props.diagram, {
    ...props.options,
    theme: osThemeRef.value === "light" ? "light" : "dark",
  }, theme.value.modalColor)
    .then((result) => {
      if (result) {
        diagramSvg.value = result.svg
        height.value = result.height
        width.value = result.width
      } else {
        diagramSvg.value = ""
        error.value = "No diagram rendered"
        height.value = 0
        width.value = 0
      }
      emits('ready', true)
    })
    .catch((err) => {
      error.value = err
      emits('ready', false)
    })
}

watch(osThemeRef, updateDiagram, { immediate: true })

watch(() => props.diagram, updateDiagram, { immediate: true })
</script>

<template>
  <div class="mermaid-renderer">
    <div v-if="error">
      <p class="error">Failed to render diagram: {{ error }}</p>
      <div class="diagram-code" v-if="diagram">
        <n-code :code="props.diagram"/>
      </div>
    </div>
    <div v-else-if="!diagramSvg" class="loading">
      Rendering diagram...
    </div>
    <div v-else-if="diagramSvg && height > 0 && width > 0" class="diagram">
      <n-image :src="diagramSrc" :width="width" :height="height" :theme-overrides="imageGroupThemeOverrides" />
    </div>
    <div v-else class="empty">
      No diagram to render
    </div>
  </div>
</template>

<style scoped>
.mermaid-renderer {
  width: 100%;
  margin: 12px 0;
  height: fit-content;
  overflow: auto;
  border-radius: v-bind('theme.borderRadius');
  border: 1px solid v-bind('theme.borderColor');
  box-sizing: border-box;
  background-color: v-bind('theme.modalColor');
}

.error {
  color: red;
  padding-left: 8px;
  padding-right: 8px;
  box-sizing: border-box;
  overflow-y: auto;
  max-height: 6em;
}

.diagram-code {
  padding: 6px 8px;
  box-sizing: border-box;
}

.loading {
  color: gray;
  padding: 8px;
  padding-left: 16px;
}

.empty {
  color: gray;
  font-style: italic;
}

.diagram {
  display: flex;
  justify-content: center;

  overflow: auto;
  padding: 8px;

  box-sizing: border-box;
  width: 100%;
  height: fit-content;
}
</style>
