<script setup lang="ts">
import { computedAsync } from '@vueuse/core'
import { useVNodeRenderer } from '../composables/useMarkdown';
import { inject } from 'vue';

const props = defineProps<{
  text: string
  mermaid?: boolean
  over?: boolean
}>()

// VNode mode
const VueMdastRenderer = inject('MarkdownRenderer') as ReturnType<typeof useVNodeRenderer>
const content = computedAsync(async () => {
  return await VueMdastRenderer(props.text, props.over ?? true)
});
</script>

<template>
  <div class="markdown-content" :style="{
    '--stream-fade-in': props.over ? 'fade-in 0.6s ease-in-out' : 'none',
  }">
    <component :is="content" />
  </div>
</template>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }

}

.markdown-content {
  width: 100%;
  height: fit-content;
  width: fit-content;
}

.markdown-content >>> *:not(strong, em) {
  animation: var(--stream-fade-in);
}


.markdown-content >>> p {
  margin: 0.2em 0;
}

.markdown-content >>> h1,
.markdown-content >>> h2,
.markdown-content >>> h3 {
  margin: 0.6em 0 0.4em 0;
}

.markdown-content >>> code {
  background-color: #f3f3f3;
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
}

.markdown-content >>> table * {
  animation: none !important;
}

.markdown-content >>> pre {
  background-color: #f5f5f5;
  padding: 1em;
  border-radius: 4px;
  overflow-x: auto;
}

.markdown-content >>> blockquote {
  border-left: 3px solid #ddd;
  padding-left: 1em;
  margin-left: 0;
  color: #666;
}

.markdown-content >>> ul,
.markdown-content >>> ol {
  padding-left: 1.5em;
  margin: 0.5em 0;
}

.markdown-content >>> .katex,
.markdown-content >>> .katex >>> * {
  user-select: none;
  -webkit-user-select: none;

  cursor: default;
}
</style>
