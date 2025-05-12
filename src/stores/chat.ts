import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Message, Conversation } from '../libs/types'
import * as Commands from '../libs/commands'

export const useChatStore = defineStore('chat', () => {
	const messages = ref<Message[]>([])
	const userInput = ref('')
	const currentConversationId = ref<string | null>(null)
	const lastMessageId = ref<string | null>(null)
	const conversations = ref<{ id: string, name: string }[]>([])

	const loadMessages = async (conversationId: string) => {
		return new Promise<void>((resolve, reject) => {
			Commands.getConversationThread(conversationId).then((storedMessages) => {
				if (storedMessages.length > 0) {
					messages.value = storedMessages;
					lastMessageId.value = storedMessages[storedMessages.length - 1]?.id || null;
				}
				resolve()
			}).catch((err) => {
				console.error('[ChatStore] Failed to load messages:', err)
				reject(err)
			})
		})
	}

	const createConversation = (name: string, description: string) => {
		return new Promise<string>((resolve, reject) => {
			Commands.createConversation(name, description)
				.then((id) => {
					conversations.value.push({ id, name })
					resolve(id)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to create conversation:', err)
					reject(err)
				})
		})
	}

	const addMessage = (message: Omit<Message, 'id'>, parentId?: string) => {
		return new Promise<string>((resolve, reject) => {
			const conversationId = currentConversationId.value
			if (!conversationId) {
				console.error('[ChatStore] No conversation selected')
				return
			}
			Commands.addMessage(conversationId, message.text, message.sender, parentId ?? (lastMessageId.value ?? undefined))
				.then(async (id) => {
					messages.value.push({ ...message, id })
					lastMessageId.value = id

					resolve(id)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to add message:', err)
					reject(err)
				})
		})
	}

	const listConversations = () => {
		return new Promise<Conversation[]>((resolve, reject) => {
			Commands.listConversations()
				.then((conversations) => {
					resolve(conversations)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to list conversations:', err)
					reject(err)
				})
		})
	}

	const updateMessage = (id: string, text: string) => {
		return new Promise<void>((resolve, reject) => {
			Commands.updateMessage(id, text)
				.then(() => {
					messages.value = messages.value.map(msg => msg.id === id ? { ...msg, text } : msg)
					resolve()
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to update message:', err)
					reject(err)
				})
		})
	}

	const deleteMessage = (id: string) => {
		return new Promise<string | null>((resolve, reject) => {
			Commands.deleteMessage(id, false)
				.then((newParent) => {
					// messages.value = messages.value.filter(msg => msg.id !== id)
					const currentConv = currentConversationId.value
					if (currentConv) loadMessages(currentConv)
					resolve(newParent)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to delete message:', err)
					reject(err)
				})
		})
	}

	const clearUserInput = () => {
		userInput.value = ''
	}

	return {
		messages,
		userInput,
		loadMessages,
		addMessage,
		createConversation,
		listConversations,
		updateMessage,
		deleteMessage,
		currentConversationId,
		// deleteMessage,
		clearUserInput
	}
})
