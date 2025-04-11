<script setup lang="ts">
import { NCard, NAvatar, NIcon } from 'naive-ui'
import MarkdownIt from 'markdown-it'
import { Chat24Regular, Person24Regular } from '@vicons/fluent'

const md = new MarkdownIt({
  breaks: true, // Convert \n to <br>
  linkify: true // Autoconvert URL-like text to links
})

function renderMarkdown(text: string): string {
  return md.render(text)
}

defineProps<{
  text: string
  sender: 'user' | 'bot'
  timestamp: Date
}>()
</script>

<template>
  <n-space align="start" :wrap-item="false" :style="{
    flexDirection: sender === 'user' ? 'row-reverse' : 'row',
    alignItems: 'flex-start'
  }">
    <n-avatar style="position: sticky; top: 0;">
      <n-icon :component="sender === 'bot' ? Chat24Regular : Person24Regular" />
    </n-avatar>
    <n-card :bordered="sender === 'user'" size="small" padding="false" :style="{
      marginLeft: sender === 'user' ? 'auto' : '0',
      maxWidth: '80%',
      backgroundColor: sender === 'user' ? '#f0f7ff' : 'transparent',
      width: 'fit-content',
      padding: '0px 16px',
      overflow: 'auto'
    }">
      <div class="markdown-content" v-html="renderMarkdown(text)"></div>
      <template #footer>
        {{ timestamp.toLocaleTimeString() }}
      </template>
    </n-card>
  </n-space>
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
