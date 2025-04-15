import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Message {
	id: string
	text: string
	sender: 'user' | 'bot'
	timestamp: Date
}

export const useChatStore = defineStore('chat', () => {
	const messages = ref<Message[]>([])
	const userInput = ref('')

	const loadMessages = async () => {
		try {
			const loaded = await invoke<Array<{id: string, text: string, sender: string, timestamp: number}>>('get_messages')
			messages.value = loaded.map(msg => ({
				...msg,
				timestamp: new Date(msg.timestamp * 1000),
				sender: msg.sender as 'user' | 'bot'
			}))
		} catch (error) {
			console.error('Failed to load messages:', error)
		}
	}

	const addMessage = async (message: Omit<Message, 'id'>) => {
		try {
			const id = crypto.randomUUID()
			const messageWithId = { ...message, id }
			await invoke('save_message', {
				id,
				text: message.text,
				sender: message.sender,
				timestamp: Math.floor(message.timestamp.getTime() / 1000)
			})
			messages.value.push(messageWithId)
		} catch (error) {
			console.error('Failed to save message:', error)
		}
	}

	const clearMessages = async () => {
		try {
			await invoke('clear_messages')
			messages.value = []
		} catch (error) {
			console.error('Failed to clear messages:', error)
		}
	}

	const clearUserInput = () => {
		userInput.value = ''
	}

	return {
		messages,
		userInput,
		loadMessages,
		addMessage,
		clearMessages,
		clearUserInput
	}
})
