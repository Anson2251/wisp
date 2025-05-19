import { clone } from 'lodash'
import { defineStore } from 'pinia'
import { ref, watch, computed } from 'vue'
import type { Message, Conversation, MessageNode } from '../libs/types'
import * as Commands from '../libs/commands'

type MessageDisplay = Message & {
	over: boolean,
	hasPrevious: boolean,
	hasNext: boolean,
}

export const useChatStore = defineStore('chat', () => {
	const userInput = ref('')
	const currentConversationId = ref<string | null>(null)
	const threadTree = ref<MessageNode | null>(null)
	const lastMessageId = ref<string | null>(null)
	const conversations = ref<{ id: string, name: string }[]>([])

	const messages = ref<Map<string, Message>>(new Map())

	const threadTreeDecisions = ref<number[]>([])

	const getDefaultThreadTreeDecisions = (root: MessageNode, prev: number[] = []) => {
		const path: number[] = []
		let currentNode = [root]
		let index = 0

		while (currentNode.length > 0) {
			const node = currentNode.shift() as MessageNode
			if (node.children.length > 0) {
				const decision = (prev[index] ?? 0) % node.children.length
				path.push(decision)
				currentNode = clone(node.children)
				index++
			}
		}

		return path;
	}

	const displayedMessages = computed(() => {
		if (threadTreeDecisions.value.length < 1) return [] as MessageDisplay[]
		if (!threadTree.value) return [] as MessageDisplay[]

		const fullDecisions = [0].concat(threadTreeDecisions.value)
		const messagesLocal: MessageDisplay[] = []

		let currentNodes = [threadTree.value]
		let index = 0

		while (currentNodes.length > 0) {
			const decision = fullDecisions[index]
			const node = currentNodes[decision]

			messagesLocal.push({
				...messages.value.get(node.message_id)!,
				over: true,
				hasNext: decision < currentNodes.length - 1,
				hasPrevious: decision > 0,
			})
			currentNodes = clone(node.children)

			index++
		}

		return messagesLocal;
	})

	const loadThreadTree = async (conversationId: string) => {
		return new Promise<MessageNode>((resolve, reject) => {
			Commands.getThreadTree(conversationId)
				.then((t) => {
					threadTree.value = t
					console.log("[ChatStore] Thread tree loaded successfully.", { conversationId })
					resolve(t)
				})
				.catch((e) => {
					console.error("[ChatStore] Fail to load the thread tree", e, {conversationId})
					reject(e)
				})
		})
	}

	const rewriteThreadTreeDecision = (decision: number[]) => {
		threadTreeDecisions.value = getDefaultThreadTreeDecisions(threadTree.value!, decision)
	}

	const changeThreadTreeDecision = (index: number, decision: number) => {
		threadTreeDecisions.value[index] = decision
	}

	const loadMessages = async (conversationId: string) => {
		return new Promise<void>((resolve, reject) => {
			Commands.getAllMessageInvolved(conversationId).then((storedMessages) => {
				if (storedMessages.length > 0) {
					messages.value.clear();
					storedMessages.forEach((m) => messages.value.set(m.id, m))
				}
				console.log("[ChatStore] Messages loaded successfully.", { conversationId })
				resolve()
			}).catch((err) => {
				console.error('[ChatStore] Failed to load messages:', err, {conversationId})
				reject(err)
			})
		})
	}

	const loadConversation = async (conversationId: string) => {
		try {
			await loadMessages(conversationId)
			await loadThreadTree(conversationId)
			threadTreeDecisions.value = getDefaultThreadTreeDecisions(threadTree.value!)
		}
		catch (err) {
			console.error('[ChatStore] Failed to load conversation:', err, { conversationId })
		}
	}

	watch(currentConversationId, (id) => {
		if (id) {
			loadMessages(id)
			loadThreadTree(id)
		}
	});

	const createConversation = (name: string, description: string) => {
		return new Promise<string>((resolve, reject) => {
			Commands.createConversation(name, description)
				.then((id) => {
					conversations.value.push({ id, name })
					console.log('[ChatStore] Conversation created successfully:', { id, name })
					resolve(id)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to create conversation:', err, { name })
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
					messages.value.set(id, { ...message, id })
					lastMessageId.value = id

					console.log('[ChatStore] Message added successfully:', { id })
					resolve(id)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to add message:', err, { message: message.text.slice(0, 20) + '...' })
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
					const originalMessage = messages.value.get(id)
					if (originalMessage) messages.value.set(id, { ...originalMessage, text })

					console.log('[ChatStore] Message updated successfully:', { id })
					resolve()
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to update message:', err, { id })
					reject(err)
				})
		})
	}

	const getMessage = (id: string) => {
		return new Promise<Message>((resolve, reject) => {
			Commands.getMessage(id)
				.then((message) => {
					console.log('[ChatStore] Message retrieved successfully:', { id })
					resolve(message)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to get message:', err, { id })
					reject(err)
				})
		})
	}

	const deleteMessage = (id: string) => {
		return new Promise<string | null>((resolve, reject) => {
			Commands.deleteMessage(id, false)
				.then((newParent) => {
					messages.value.delete(id)
					lastMessageId.value = newParent

					console.log('[ChatStore] Message deleted successfully:', { id })
					resolve(newParent)
				})
				.catch((err) => {
					console.error('[ChatStore] Failed to delete message:', err, { id })
					reject(err)
				})
		})
	}

	const clearUserInput = () => {
		userInput.value = ''
	}

	return {
		messages,
		threadTree,
		userInput,
		loadMessages,
		addMessage,
		getMessage,
		createConversation,
		listConversations,
		updateMessage,
		deleteMessage,
		currentConversationId,
		loadThreadTree,
		clearUserInput,

		changeThreadTreeDecision,
		rewriteThreadTreeDecision,
		threadTreeDecisions,
		displayedMessage: displayedMessages,
		loadConversation,
	}
})
