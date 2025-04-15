<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue'

const props = defineProps<{
  autoScroll?: boolean
}>()

const container = ref<HTMLElement | null>(null)
const throttlePixel = 48

const scrollToBottom = (withThrottle = true) => {
  if (!container.value) return

  if (container.value.scrollTop + container.value.clientHeight ===
    container.value.scrollHeight) return

  if ((container.value.scrollTop + container.value.clientHeight <
    container.value.scrollHeight - throttlePixel) && withThrottle) return

  container.value.scrollTo({
    top: container.value.scrollHeight,
    behavior: 'smooth'
  })
}

// Auto-scroll when slot content changes (if enabled)
watch(() => container.value?.children, () => {
  if (props.autoScroll !== false) {
    scrollToBottom()
  }
}, { deep: true })

// Initial scroll to bottom (if enabled)
onMounted(() => {
  if (props.autoScroll !== false) {
    scrollToBottom()
  }
})

defineExpose({
  scrollToBottom
})
</script>

<template>
  <div ref="container" style="height: 100%; overflow: auto;">
    <div style="width: 100%; height: fit-content;">
      <slot />
    </div>
  </div>
</template>
