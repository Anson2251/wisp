<script setup lang="ts">
import { NModal, NButton, NButtonGroup, useMessage } from 'naive-ui';
import { Codemirror } from 'vue-codemirror';
import { markdown } from '@codemirror/lang-markdown'
import { ref, watch } from 'vue';
import { useChatStore } from '../stores/chat';

const message = useMessage()
const chat = useChatStore()

const props = defineProps<{
  show: boolean,
  id: string,
}>()
const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
}>()

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

const setLoadingState = (loading: boolean, resend: boolean = false) => {
  if (resend) loadingResend.value = loading
  else loadingSave.value = loading
}

watch(() => props.show, (newValue) => {
  if (newValue) loadMessage()
})

const updateMessageLocal = (id: string, text: string, resend = false) => {
  setLoadingState(true, resend)

  chat.updateMessage(id, text).then(() => {
    setLoadingState(false, resend)

    setTimeout(() => emit('update:show', false))

    if (resend) {
      message.error("Not implemented yet")
    }
  }).catch((error) => {
    console.error("Failed to update message:", error)
  })
}
</script>

<template>
  <n-modal v-bind:show="show" v-on:update:show="value => emit('update:show', value)" class="custom-card"
    :preset="'card'" :draggable="true" title="Edit Message" :bordered="false" size="small" :content-style="{
      minHeight: 0,
      minWidth: 0,
    }"
    :style="{
      width: '600px',
      height: '400px',
    }">
    <div class="editor-container">
    <codemirror
      auto-focus
      v-model="code"
      style="height: 100%; width: 100%;"
      :extensions="[markdown()]"

    />
    </div>
    <template #footer>
      <div class="footer">
        <n-button @click="emit('update:show', false)" >Cancel</n-button>
        <n-button-group>
          <n-button @click="updateMessageLocal(props.id, code, false)" type="primary" :loading="loadingSave">Save</n-button>
          <n-button @click="updateMessageLocal(props.id, code, true)" type="primary" :loading="loadingResend">Resend</n-button>
        </n-button-group>
      </div>
    </template>
  </n-modal>
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

.footer {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  height: fit-content;
}
</style>
