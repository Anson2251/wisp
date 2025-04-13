<script setup lang="ts">
import { watch, ref, inject } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface MarkdownProcessor {
  remark: { process: (text: string) => Promise<{ toString: () => string }> };
  rehype: { process: (text: string) => Promise<{ toString: () => string }> };
}

const props = defineProps<{
  text: string
}>()

const { remark, rehype } = inject<MarkdownProcessor>('markdownProcessor')!

async function hashContent(text: string): Promise<string> {
  return await invoke<string>('hash_content', { content: text });
}

async function extractAndCacheDiagrams(text: string): Promise<string> {
  try {
    // Split text into non-mermaid and mermaid parts
    const parts = text.split(/```mermaid([\s\S]*?)```/g);
    let processed = '';

    for (let i = 0; i < parts.length; i++) {
      if (i % 2 === 0) {
        // Regular text
        processed += await remark.process(parts[i]);
      } else {
        console.log('cache hit for mermaid');
        // Mermaid diagram
        const diagram = parts[i].trim();
        const hash = await hashContent(diagram);
        let cached = await invoke<string>('get_cached_render', { hash });

        if (!cached) {
          // Render and cache if not found
          let result = await remark.process(`\`\`\`mermaid\n${diagram}\n\`\`\``);
          result = await rehype.process(result.toString());
          cached = result.toString();
          await invoke<void>('put_cached_render', {
            hash,
            astJson: diagram,
            renderedHtml: cached
          });
        }
        processed += cached;
      }
    }
    return processed;
  } catch (error) {
    console.error("Error processing diagrams:", error);
    return text;
  }
}

const renderedText = ref("")
watch(() => props.text, async (newText) => {
  renderedText.value = await extractAndCacheDiagrams(newText)
}, { immediate: true })
</script>

<template>
  <div class="markdown-content" v-html="renderedText"></div>
</template>

<style scoped>
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
