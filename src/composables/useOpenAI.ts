import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

export function useOpenAI() {
	const isStreaming = ref(false)

	const streamResponse = async (
		messages: any[],
		onChunk: (chunk: string) => void,
		onFinish: () => void
	) => {
		isStreaming.value = true
		const unlisten = await listen<string>('openai_stream_chunk', (event) => {
			onChunk(event.payload)
		})

		try {
			await invoke('ask_openai_stream', { messages })
		} finally {
			unlisten()
			isStreaming.value = false
			if(onFinish) onFinish()
		}
	}

	return {
		isStreaming,
		streamResponse
	}
}
