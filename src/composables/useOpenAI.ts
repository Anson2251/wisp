import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'
import { INTERFACE_PROMPT, INTERFACE_REGENERATE_INSERT } from '../prompt-management/constants/interfacePrompt'
import { cloneDeep } from 'lodash'

export function useOpenAI() {
	const isStreaming = ref(false)

	const streamResponse = async (
		messages: any[],
		onContentChunk: (chunk: string) => void,
		onReasoningChunk: (chunk: string) => void,
		onFinish: () => void,
		ignoreLastMessage: boolean = false,
		insertRegenerateGuidancePrompt: boolean = false,
	): Promise<void> => {
		isStreaming.value = true
		const unlistenContent = await listen<string>('openai_stream_chunk', (event) => {
			onContentChunk(event.payload)
		})

		const unlistenReasoning = await listen<string>('openai_stream_chunk_reasoning', (event) => {
			onReasoningChunk(event.payload)
		})

		try {
			messages = cloneDeep(messages)
			messages.unshift({role: "system", content: INTERFACE_PROMPT})
			if (ignoreLastMessage) messages = messages.slice(0, -1)
			if (insertRegenerateGuidancePrompt) messages.push({role: "system", content: INTERFACE_REGENERATE_INSERT})

			if (messages.length > 0) await invoke('ask_openai_stream', { messages })
			else console.warn('[useOpenAI] No messages to stream')
		}
		catch (error) {
			console.error('[useOpenAI] Error streaming response:', error)
			return Promise.reject("Fail to stream response: " + error)
		}
		 finally {
			unlistenContent()
			unlistenReasoning()
			isStreaming.value = false
			if(onFinish) onFinish()
		}
	}

	return {
		isStreaming,
		streamResponse
	}
}
