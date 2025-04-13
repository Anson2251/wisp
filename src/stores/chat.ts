import { defineStore } from 'pinia'
import { ref } from 'vue'

interface Message {
	text: string
	sender: 'user' | 'bot'
	timestamp: Date
}

export const useChatStore = defineStore('chat', () => {
	const messages = ref<Message[]>([])
	const userInput = ref('')

	const addMessage = (message: Message) => {
		messages.value.push(message)
	}

	const clearUserInput = () => {
		userInput.value = ''
	}

	return {
		messages,
		userInput,
		addMessage,
		clearUserInput
	}
})
