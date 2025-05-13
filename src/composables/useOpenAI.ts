import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'
import { INTERFACE_PROMPT } from '../prompt-management/constants/interfacePrompt'

export function useOpenAI() {
	const isStreaming = ref(false)

	const streamResponse = async (
		messages: any[],
		onChunk: (chunk: string) => void,
		onFinish: () => void,
		ignoreLastMessage: boolean = false
	) => {
		isStreaming.value = true
		const unlisten = await listen<string>('openai_stream_chunk', (event) => {
			onChunk(event.payload)
		})

		try {
			await invoke('ask_openai_stream', { messages: [{role: "system", content: INTERFACE_PROMPT}, ...(messages.slice(0, ignoreLastMessage ? -1 : undefined))] })
		}
		catch (error) {
			console.error('Error streaming response:', error)
		}
		 finally {
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
