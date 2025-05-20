<script setup lang="ts">
import { NTooltip } from "naive-ui";
import {computed, ref} from "vue";

const props = defineProps<{
  textToCopy: string,
  textCopied: string,
  onCopy: () => void | Promise<void>,
}>()

const copied = defineModel<boolean>("show", {required: false, default: false})

const copyCode = async () => {
  await props.onCopy?.()
  copied.value = true
  // prevent the copied indicator changing during the animation
  const restoreCopiedIndicator = setInterval(() => {
    if (!copiedToolTipShow.value) {
      clearInterval(restoreCopiedIndicator)
      setTimeout(() => copied.value = false, 150)
    }
  }, 100)
}

const copiedToolTipShow = ref(false)
const copyIndicator = computed(() => {
  return copied.value ? props.textCopied  : props.textToCopy
})
</script>

<template>
  <n-tooltip trigger="hover" v-model:show="copiedToolTipShow">
    <template #trigger>
      <div @click="copyCode">
        <slot name="trigger"></slot>
      </div>
    </template>
    <span>{{ copyIndicator }}</span>
  </n-tooltip>
</template>
