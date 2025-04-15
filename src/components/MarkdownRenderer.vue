<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { computedAsync } from '@vueuse/core'
import { useMarkdown, useVueMdastRenderer } from '../composables/useMarkdown';

const {
  useStreamingMarkdownRenderer,
  injectMarkdownProcessor
} = useMarkdown();

type RenderMode = 'html' | 'stream' | 'vnode';

const props = defineProps<{
  text: string
  mode?: RenderMode
}>()

const mode = props.mode ?? 'html';
const container = ref<HTMLDivElement | null>(null);

// HTML mode (default)
const { processMarkdown } = injectMarkdownProcessor();
const html = computedAsync(async () => {
  return await processMarkdown(props.text)
});

// VNode mode
const VueMdastRenderer = useVueMdastRenderer();
const content = computed(() => {
  return VueMdastRenderer(props.text)
});

// Stream mode
const toAppend = useStreamingMarkdownRenderer((html) => {
  if(!container.value) return;
  container.value.innerHTML += html;
});
if(mode === 'stream') {
  watch(() => props.text, (newText) => {
    toAppend(newText);
  });
}
</script>

<template>
  <div v-if="mode === 'html'" class="markdown-content" v-html="html"></div>
  <div v-else-if="mode === 'stream'" class="markdown-content" ref="container"></div>
  <component v-else-if="mode === 'vnode'" :is="content"></component>
</template>

<style>
@keyframes fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }

}

.markdown-content > div {
  animation: fade-in 0.5s ease-in-out;
  width: 100%;
  height: fit-content;
}


.markdown-content p {
  margin: 0.2em 0;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3 {
  margin: 0.6em 0 0.4em 0;
}

.markdown-content code {
  background-color: #f3f3f3;
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
}

.markdown-content pre {
  background-color: #f5f5f5;
  padding: 1em;
  border-radius: 4px;
  overflow-x: auto;
}

.markdown-content blockquote {
  border-left: 3px solid #ddd;
  padding-left: 1em;
  margin-left: 0;
  color: #666;
}

.markdown-content ul,
.markdown-content ol {
  padding-left: 1.5em;
  margin: 0.5em 0;
}
</style>
