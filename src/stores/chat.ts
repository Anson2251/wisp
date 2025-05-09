import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export type Message = {
	id: string
	text: string
	sender: 'user' | 'bot'
	timestamp: Date
}

type MessageStored = {
	id: string
	text: string
	sender: string
	timestamp: number
}

export const useChatStore = defineStore('chat', () => {
	const messages = ref<Message[]>([])
	const userInput = ref('')
	const currentConversationId = ref<string>('default')
	const conversations = ref<{id: string, name: string}[]>([])

	const loadMessages = async () => {
		try {
			const loaded = await invoke<MessageStored[]>('get_messages', {
				conversationId: currentConversationId.value
			})
			messages.value = loaded.map(msg => ({
				...msg,
				timestamp: new Date(msg.timestamp * 1000),
				sender: msg.sender as 'user' | 'bot'
			}))
		} catch (error) {
			console.error('[ChatStore] Failed to load messages:', error)
		}
	}

	const createConversation = async (name: string) => {
		try {
			const id = await invoke<string>('create_conversation', { name })
			currentConversationId.value = id
			messages.value = []
			return id
		} catch (error) {
			console.error('[ChatStore] Failed to create conversation:', error)
			throw error
		}
	}

	const saveMessage = async (message: Message) => {
		try {
			await invoke('add_message', {
				conversationId: currentConversationId.value,
				text: message.text,
				sender: message.sender,
				parentId: null,
				timestamp: Math.floor(message.timestamp.getTime() / 1000)
			})
		} catch (error) {
			console.error('[ChatStore] Failed to save message:', error)
		}
	}

	const addMessage = async (message: Omit<Message, 'id'>, store = true) => {
		try {
			const id = crypto.randomUUID().toLocaleUpperCase()
			const messageWithId = { ...message, id }
			if (store) await saveMessage(messageWithId)
			messages.value.push(messageWithId)
		} catch (error) {
			console.error('[ChatStore] Failed to add message:', error)
		}
	}

	const clearMessages = async () => {
		try {
			await invoke('clear_messages')
			messages.value = []
		} catch (error) {
			console.error('[ChatStore] Failed to clear messages:', error)
		}
	}

	const deleteMessage = async (id: string) => {
		try {
			await invoke('delete_message', { id })
			messages.value = messages.value.filter(msg => msg.id !== id)
		} catch (error) {
			console.error('[ChatStore] Failed to delete message:', error)
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
		saveMessage,
		clearMessages,
		deleteMessage,
		clearUserInput
	}
})
