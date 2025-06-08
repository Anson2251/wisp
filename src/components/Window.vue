<script lang="ts" setup>
import { ref, onUnmounted } from 'vue'
import { useThemeVars, NButton, NButtonGroup, NIcon, NText } from 'naive-ui'
import { clamp } from 'lodash'
import { Dismiss16Filled, ArrowMaximize16Filled, ArrowMinimize16Filled } from '@vicons/fluent'

const emit = defineEmits(['close'])

const props = defineProps({
  title: {
    type: String,
    default: 'Untitled Window'
  },
  initialWidth: {
    type: Number,
    default: 600
  },
  initialHeight: {
    type: Number,
    default: 400
  },
  initialX: {
    type: Number,
  },
  initialY: {
    type: Number,
  },
  zIndex: {
    type: Number,
    default: 1
  },
  allowMaximised: {
    type: Boolean,
    default: true
  }
})

const themeVars = useThemeVars()
const width = ref(props.initialWidth)
const height = ref(props.initialHeight)
const x = ref(props.initialX ?? (window.innerWidth - width.value) / 2)
const y = ref(props.initialY ?? (window.innerHeight - height.value) / 2)
const isMaximized = ref(false)
const originalState = ref({
  width: props.initialWidth,
  height: props.initialHeight,
  x: props.initialX ?? (window.innerWidth - props.initialWidth) / 2,
  y: props.initialY ?? (window.innerHeight - props.initialHeight) / 2
})

const toggleMaximize = (e: MouseEvent) => {
  if (!props.allowMaximised)  {
    isMaximized.value = false
    return
  }
  e.stopPropagation()

  if (isMaximized.value) {
    // Restore original size/position
    width.value = originalState.value.width
    height.value = originalState.value.height
    x.value = originalState.value.x
    y.value = originalState.value.y
  } else {
    // Save current state and maximize
    originalState.value = {
      width: width.value,
      height: height.value,
      x: x.value,
      y: y.value
    }
    width.value = window.innerWidth
    height.value = window.innerHeight
    x.value = 0
    y.value = 0
  }
  isMaximized.value = !isMaximized.value
}

const handleClose = (e : MouseEvent) => {
  e.stopPropagation()
  emit('close')
}


let isDragging = false
let startX = 0
let startY = 0

let isResizing = false
type Directions = 'south' | 'west' | 'east' | 'north' | 'south-west' | 'north-west' | 'south-east' | 'north-east' | ''
let resizeDirection: Directions = ''
let startWidth = 0
let startHeight = 0

const startDrag = (e: MouseEvent) => {
  e.preventDefault()
  e.stopPropagation()

  isDragging = true
  startX = e.clientX - x.value
  startY = e.clientY - y.value
  document.addEventListener('mousemove', drag)
  document.addEventListener('mouseup', stopDrag)
}

const drag = (e: MouseEvent) => {
  if (!isDragging) return

  const newX = e.clientX - startX
  const newY = e.clientY - startY
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight

  // Ensure window stays within bounds with margin
  x.value = clamp(newX, 0, viewportWidth - width.value)
  y.value = clamp(newY, 0, viewportHeight - height.value)
}

const stopDrag = () => {
  isDragging = false
  document.removeEventListener('mousemove', drag)
  document.removeEventListener('mouseup', stopDrag)
}

const startResize = (e: MouseEvent, direction: Directions) => {
  e.preventDefault()
  e.stopPropagation()

  isResizing = true
  resizeDirection = direction
  startX = e.clientX
  startY = e.clientY
  startWidth = width.value
  startHeight = height.value
  document.addEventListener('mousemove', resize)
  document.addEventListener('mouseup', stopResize)
}

window.onresize = () => {
  const viewportHeight = window.innerHeight
  const viewportWidth = window.innerWidth

  if (!isMaximized.value) {
    height.value = clamp(height.value, 100, viewportHeight)
    width.value = clamp(width.value, 100, viewportWidth)
  }
  else {
    height.value = viewportHeight
    width.value = viewportWidth
  }

  if (x.value + width.value > viewportWidth) {
    x.value = viewportWidth - width.value
  }
  if (y.value + height.value > viewportHeight) {
    y.value = viewportHeight - height.value
  }
}

const resize = (e: MouseEvent) => {
  if (!isResizing) return

  const dx = e.clientX - startX
  const dy = e.clientY - startY

  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight

  const adjust = {
    "north": () => {
      const newHeight = Math.max(100, startHeight - dy)
      y.value = clamp(y.value - (newHeight - height.value), 0, viewportHeight - height.value)
      height.value = Math.max(100, startHeight - dy)
    },
    "south": () => {
      height.value = Math.max(100, Math.min(
        startHeight + dy,
        viewportHeight - y.value - 10 // Leave some margin
      ))
    },
    "east": () => {
      width.value = Math.max(100, Math.min(
        startWidth + dx,
        viewportWidth - x.value - 10 // Leave some margin
      ))
    },
    "west": () => {
      const newWidth = Math.max(100, startWidth - dx)
      x.value = clamp(x.value - (newWidth - width.value), 0, viewportWidth - width.value)
      width.value = newWidth
    }
  }

  resizeDirection.split("-").forEach(direction => {
    adjust[direction as keyof typeof adjust]()
  })
}

const stopResize = () => {
  isResizing = false
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
}

onUnmounted(() => {
  document.removeEventListener('mousemove', drag)
  document.removeEventListener('mouseup', stopDrag)
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
})
</script>

<template>
  <div class="window" :style="{
    left: `${x}px`,
    top: `${y}px`,
    width: `${width}px`,
    height: `${height}px`,
    border: isMaximized ? 'none' : `1px solid ${themeVars.borderColor}`,
    zIndex
  }">
    <!-- Title Bar (draggable) -->
    <div class="title-bar" @mousedown="startDrag">
      <n-text strong>{{ title  }}</n-text>
      <n-button-group class="window-controls">
        <n-button quaternary @click.stop="toggleMaximize" size="small" @mousedown="(e) => e.stopPropagation()" v-if="allowMaximised">
          <template #icon>
            <n-icon :component="isMaximized ? ArrowMinimize16Filled : ArrowMaximize16Filled" size="16"
              class="control-icon" :color="themeVars.textColorBase"/>
          </template>
        </n-button>
        <n-button quaternary @click.stop="handleClose" size="small" @mousedown="(e) => e.stopPropagation()">
          <template #icon>
            <n-icon :component="Dismiss16Filled" size="16" class="control-icon" :color="themeVars.textColorBase" />
          </template>
        </n-button>
      </n-button-group>
    </div>

    <!-- Content Slot -->
    <div class="content">
      <slot></slot>
    </div>

    <!-- Footer Slot -->
    <div class="footer" v-if="$slots.footer">
      <slot name="footer"></slot>
    </div>

    <!-- Resize Handles -->
    <div class="resize-handle top" @mousedown="(e) => startResize(e, 'north')"></div>
    <div class="resize-handle right" @mousedown="(e) => startResize(e, 'east')"></div>
    <div class="resize-handle bottom" @mousedown="(e) => startResize(e, 'south')"></div>
    <div class="resize-handle left" @mousedown="(e) => startResize(e, 'west')"></div>

    <div class="resize-handle top-left" @mousedown="(e) => startResize(e, 'north-west')"></div>
    <div class="resize-handle top-right" @mousedown="(e) => startResize(e, 'north-east')"></div>
    <div class="resize-handle bottom-left" @mousedown="(e) => startResize(e, 'south-west')"></div>
    <div class="resize-handle bottom-right" @mousedown="(e) => startResize(e, 'south-east')"></div>
  </div>
</template>

<style scoped>
.window {
  position: fixed;
  background-color: v-bind('themeVars.cardColor');
  color: v-bind('themeVars.textColor1');
  border-radius: v-bind('isMaximized ? 0 : themeVars.borderRadius');
  box-shadow: v-bind('themeVars.boxShadow1');
  display: flex;
  flex-direction: column;
  overflow: hidden;

  box-sizing: border-box;
  transition: background-color, color, border-radius, box-shadow 0.2s v-bind('themeVars.cubicBezierEaseInOut');
}

.title-bar {
  padding: 4px 4px 4px 12px;
  background-color: v-bind('themeVars.tableHeaderColor');
  cursor: move !important;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.title-bar:deep(*):not(button, button:deep(*)) {
  cursor: move;
}

.window-controls {
  display: flex;
  gap: 8px;
}

.control-icon {
  cursor: pointer;
  opacity: 0.7;
  transition: opacity 0.2s v-bind('themeVars.cubicBezierEaseIn');
}

.control-icon:hover {
  opacity: 1;
}

.content {
  flex: 1;
  overflow: auto;
}

.footer {
  padding: 6px 8px;
  border-top: 1px solid v-bind('themeVars.borderColor');
}

.resize-handle {
  position: absolute;
  background: transparent;
  z-index: 10;
}

.top {
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  cursor: ns-resize !important;
}

.right {
  top: 0;
  right: 0;
  bottom: 0;
  width: 4px;
  cursor: ew-resize !important;
}

.bottom {
  bottom: 0;
  left: 0;
  right: 0;
  height: 4px;
  cursor: ns-resize !important;
}

.left {
  top: 0;
  left: 0;
  bottom: 0;
  width: 4px;
  cursor: ew-resize !important;
}

.top-left {
  top: 0;
  left: 0;
  width: 6px;
  height: 6px;
  cursor: nwse-resize !important;
}
.top-right {
  top: 0;
  right: 0;
  width: 6px;
  height: 6px;
  cursor: nesw-resize !important;
}
.bottom-left {
  bottom: 0;
  left: 0;
  width: 6px;
  height: 6px;
  cursor: nesw-resize !important;
}
.bottom-right {
  bottom: 0;
  right: 0;
  width: 6px;
  height: 6px;
  cursor: nwse-resize !important;
}
</style>
