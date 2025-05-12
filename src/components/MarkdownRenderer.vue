<script setup lang="ts">
import { computedAsync } from '@vueuse/core'
import { useVNodeRenderer } from '../composables/useMarkdown';

const props = defineProps<{
  text: string
  mermaid?: boolean
  over?: boolean
}>()

// VNode mode
const VueMdastRenderer = useVNodeRenderer();
const content = computedAsync(async () => {
  return await VueMdastRenderer(props.text, props.over ?? true)
});
</script>

<template>
  <div style="width: 100%; height: fit-content;" class="markdown-content">
    <component :is="content"/>
  </div>
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

.markdown-content *:not(strong, em) {
  animation: fade-in 0.8s ease-in-out;
  /* transition: all 0.3s ease-in-out; */
}

.markdown-content > * {
  width: 100%;
  height: fit-content;
  box-sizing: border-box;
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

/* if there is only one child (class name MathJax)*/
/* .markdown-content p > .MathJax:only-child {
  display: block;
  width: 100%;
  margin: 0 auto;
  text-align: center;
  font-size: 1.2em;
} */

.markdown-content ul,
.markdown-content ol {
  padding-left: 1.5em;
  margin: 0.5em 0;
}
</style>
