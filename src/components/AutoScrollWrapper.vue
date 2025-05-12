<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue'

const props = defineProps<{
  auto?: boolean
  throttle?: number
  smooth?: boolean
}>()

const container = ref<HTMLElement | null>(null)
const throttlePixel = props.throttle ?? -1

const scrollToBottom = (withThrottle = true) => {
  if (!container.value) return

  if (container.value.scrollTop + container.value.clientHeight ===
    container.value.scrollHeight) return

  if ((container.value.scrollTop + container.value.clientHeight <
    container.value.scrollHeight - (throttlePixel > 0 ? throttlePixel : container.value.scrollHeight)) && withThrottle) return

  container.value.scrollTo({
    top: container.value.scrollHeight,
    behavior: props.smooth ? 'smooth' : 'auto'
  })
}

// Auto-scroll when slot content changes (if enabled)
watch(() => container.value?.children, () => {
  if (props.auto !== false) {
    scrollToBottom()
  }
}, { deep: true })

// Initial scroll to bottom (if enabled)
onMounted(() => {
  if (props.auto !== false) {
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
