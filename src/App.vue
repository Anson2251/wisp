<script setup lang="ts">
import Chat from './components/Chat.vue'
import { NDialogProvider, NConfigProvider } from 'naive-ui';
import katex from 'katex'
import hljs from 'highlight.js/lib/core'

import javascript from 'highlight.js/lib/languages/javascript'
import typescript from 'highlight.js/lib/languages/typescript'
import html from 'highlight.js/lib/languages/xml'
import css from 'highlight.js/lib/languages/css'
import python from 'highlight.js/lib/languages/python'
import java from 'highlight.js/lib/languages/java'
import cpp from 'highlight.js/lib/languages/cpp'
import csharp from 'highlight.js/lib/languages/csharp'
import php from 'highlight.js/lib/languages/php'
import ruby from 'highlight.js/lib/languages/ruby'
import go from 'highlight.js/lib/languages/go'
import rust from 'highlight.js/lib/languages/rust'
import swift from 'highlight.js/lib/languages/swift'
import sql from 'highlight.js/lib/languages/sql'
import bash from 'highlight.js/lib/languages/bash'
import json from 'highlight.js/lib/languages/json'
import yaml from 'highlight.js/lib/languages/yaml'
import markdown from 'highlight.js/lib/languages/markdown'
import prolog from 'highlight.js/lib/languages/prolog'
import shell from 'highlight.js/lib/languages/shell'
import dockerfile from 'highlight.js/lib/languages/dockerfile'
import vue from 'highlight.js/lib/languages/vue'
import kotlin from 'highlight.js/lib/languages/kotlin'
import perl from 'highlight.js/lib/languages/perl'
import lua from 'highlight.js/lib/languages/lua'
import r from 'highlight.js/lib/languages/r'

hljs.registerLanguage('javascript', javascript)
hljs.registerLanguage('typescript', typescript)
hljs.registerLanguage('html', html)
hljs.registerLanguage('css', css)
hljs.registerLanguage('python', python)
hljs.registerLanguage('java', java)
hljs.registerLanguage('cpp', cpp)
hljs.registerLanguage('csharp', csharp)
hljs.registerLanguage('php', php)
hljs.registerLanguage('ruby', ruby)
hljs.registerLanguage('go', go)
hljs.registerLanguage('rust', rust)
hljs.registerLanguage('swift', swift)
hljs.registerLanguage('sql', sql)
hljs.registerLanguage('bash', bash)
hljs.registerLanguage('json', json)
hljs.registerLanguage('yaml', yaml)
hljs.registerLanguage('markdown', markdown)
hljs.registerLanguage('prolog', prolog)
hljs.registerLanguage('shell', shell)
hljs.registerLanguage('sh', shell)

hljs.registerLanguage('dockerfile', dockerfile)
hljs.registerLanguage('kotlin', kotlin)
hljs.registerLanguage('perl', perl)
hljs.registerLanguage('lua', lua)
hljs.registerLanguage('r', r)

</script>

<template>
  <n-dialog-provider>
    <n-config-provider :katex="(katex as any)" :hljs="hljs">
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
