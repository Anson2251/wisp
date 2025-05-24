<script setup lang="ts">
import { NButton, NButtonGroup, useOsTheme } from 'naive-ui';
import Window from './Window.vue';
import { Codemirror } from 'vue-codemirror';
import { Compartment } from '@codemirror/state';
import { oneDark } from '@codemirror/theme-one-dark'
import { vsCodeLight } from '@fsegurai/codemirror-theme-vscode-light';
import { basicSetup, EditorView } from 'codemirror'
import { markdown } from '@codemirror/lang-markdown'
import { ref, watch } from 'vue';
import { useChatStore } from '../stores/chat';

const chat = useChatStore()

const props = defineProps<{
  show: boolean,
  id: string,
}>()
const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'resend', derive: boolean, text: string): void
}>()

const editorTheme = new Compartment();
const osThemeRef = useOsTheme()
const extensions = [markdown(), basicSetup, editorTheme.of(osThemeRef.value === 'dark' ? oneDark : vsCodeLight)]

const handleReady = (payload: any) => {
  const cmEditor = payload.view as EditorView
  watch(osThemeRef, (theme) => {
    cmEditor.dispatch({
      effects: editorTheme.reconfigure(theme === 'dark' ? oneDark : vsCodeLight)
    })
  })
}

const code = ref<string>("")
const loadingSave = ref(false)
const loadingResend = ref(false)

const loadMessage = () => {
  chat.getMessage(props.id).then((message) => {
    if (!message) {
      console.error("Message not found")
      return
    }
    code.value = message.text.trim() + "\n"
  }).catch((error) => {
    console.error("Failed to get message:", error)
  })
}

const setLoadingState = (loading: boolean) => {
  loadingResend.value = loading
}

const closeEditor = () => {
  emit('update:show', false)
}

watch(() => props.show, (newValue) => {
  if (newValue) loadMessage()
})

const updateMessageLocal = (text: string) => {
  setLoadingState(true)

  chat.updateMessage(props.id, text)
    .then(() => {
      setLoadingState(false)
      closeEditor()
    })
    .catch((error) => {
      console.error("Failed to update message:", error)
    })
}

const resendMessageLocal = (text: string) => {
  emit('resend', true, text)
  closeEditor()
}
</script>

<template>
  <window v-if="show" @close="closeEditor" title="Message Editor">
    <template #default>
      <div class="editor-container">
        <codemirror auto-focus v-model="code" style="height: 100%; width: 100%;" :extensions="extensions"
          @ready="handleReady" />
      </div>
    </template>
    <template #footer>
      <div class="footer">
        <n-button @click="closeEditor">Cancel</n-button>
        <n-button-group>
          <n-button @click="updateMessageLocal(code)" type="primary"
            :loading="loadingSave">Save</n-button>
          <n-button @click="resendMessageLocal(code)" type="primary"
            :loading="loadingResend">Resend</n-button>
        </n-button-group>
      </div>
    </template>
  </window>
</template>

<style scoped>
.editor-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;

  min-width: none;
  min-height: none;
}

.editor-container:deep(*) {
  cursor: auto;
  user-select: auto;
  -webkit-user-select: auto;
}

.footer {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  height: fit-content;
}
</style>
