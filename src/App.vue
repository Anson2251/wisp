<script setup lang="ts">
import Chat from './components/Chat.vue'
import { NDialogProvider, NConfigProvider } from 'naive-ui';
import katex from 'katex'

</script>

<template>
  <n-dialog-provider>
    <n-config-provider :katex="(katex as any)">
      <Chat />
    </n-config-provider>
  </n-dialog-provider>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}


html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>
