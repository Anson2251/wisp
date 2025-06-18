import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'
import { INTERFACE_PROMPT, INTERFACE_REGENERATE_INSERT } from '../prompt-management/constants/interfacePrompt'
import { cloneDeep } from 'lodash'
import { getUrl } from '../libs/commands'
import type { Model, Provider } from '../libs/types'

export function useOpenAI() {
	const isStreaming = ref(false)

	const streamResponse = async (
		messages: any[],
		model: string,
		provider: Provider,
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

			if (messages.length > 0) await invoke('ask_openai_stream', {
				messages,
				model,
				provider
			})
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

	const fetchModels = async (baseUrl: string, apiKey: string): Promise<Model[]> => {
		try {
			const response = await getUrl({
				url: `${baseUrl}/models`,
				headers: {
					'Authorization': `Bearer ${apiKey}`,
					'Content-Type': 'application/json'
				},
				parseJson: true
			})

			if (!response.data || !Array.isArray(response.data)) {
				throw new Error('Invalid models response format')
			}

			return response.data.map((model: any) => ({
				metadata: {
					name: model.id,
					display_name: model.id,
					description: model.description || ''
				},
				model_info: {
					type: 'text_generation',
					configs: {
						parameters: {},
						capabilities: [],
						multimodal: {
							text: {
								context_window: model.context_window || 2048,
								languages: ['en']
							}
						}
					}
				},
				max_input_size: model.context_window || 2048
			}))
		} catch (error) {
			console.error('[useOpenAI] Error fetching models:', error)
			throw new Error(`Failed to fetch models: ${error}`)
		}
	}

	return {
		isStreaming,
		streamResponse,
		fetchModels
	}
}
