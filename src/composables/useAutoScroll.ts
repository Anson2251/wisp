import { onMounted, ref, watch } from 'vue'

export function useAutoScroll(containerId: string, throttleMs = 48) {
	const container = ref<HTMLElement | null>(null)

	const scrollToBottom = (withThrottle = true) => {
		if (!container.value) {
			const el = document.getElementById(containerId)
			if (el) container.value = el.parentElement as HTMLElement
		}

		if (container.value) {
			if (container.value.scrollTop + container.value.clientHeight ===
				container.value.scrollHeight) return

			if ((container.value.scrollTop + container.value.clientHeight <
				container.value.scrollHeight - throttleMs) && withThrottle) return

			container.value.scrollTo({
				top: container.value.scrollHeight,
				behavior: 'smooth'
			})
		}
	}

	const setupAutoScroll = (target: any) => {
		onMounted(() => {
			watch(target, () => scrollToBottom(), { deep: true })
		})
	}

	return {
		scrollToBottom,
		setupAutoScroll
	}
}
