import { defineStore } from 'pinia'
import { ref, watch, type ComputedRef, computed } from 'vue'
import type { Message, Conversation } from '../libs/types'
import * as Commands from '../libs/commands'

type MessageDisplay = Omit<Message, 'text'> & {
	over: boolean,
	hasPrevious: boolean,
	hasNext: boolean,
	text: ComputedRef<string>
}

export const useChatStore = defineStore('chat', () => {
	const userInput = ref('')
	const currentConversationId = ref<string | null>(null)
	const threadTree = ref(new Map<string, readonly string[]>())
	const lastMessageId = ref<string | null>(null)
	const rootMessageId = ref<string | null>(null)
	const conversations = ref<{ id: string, name: string }[]>([])

	const messages = ref<Map<string, Message>>(new Map())

	const threadTreeDecisions = ref<number[]>([])

	const getDefaultThreadTreeDecisions = (root: string, prev: number[] = []) => {
		const path: number[] = []
		let node = root
		let index = 0

		while (node) {
			const children = threadTree.value.get(node) ?? []
			if (children.length > 0) {
				const decision = (prev[index] ?? 0) % children.length
				node = children[decision]
				path.push(decision)
				index++
			}
			else {
				break
			}
		}

		return path;
	}

	const displayedMessages = ref<MessageDisplay[]>([])
	watch([threadTreeDecisions, threadTree, rootMessageId, lastMessageId], () => {
		if (threadTreeDecisions.value.length < 1
			|| !threadTree.value
			|| !rootMessageId.value
		) {
			console.log("[ChatStore] No decisions or tree or root message id")
			return [] as MessageDisplay[]
		}

		const timingIdentifier = "[ChatStore] Displayed messages re-computed"
		console.time(timingIdentifier)

		const fullDecisions = Object.freeze(threadTreeDecisions.value)
		const messagesLocal: MessageDisplay[] = []

		const getNode = (id: string, hasNext: boolean, hasPrevious: boolean): MessageDisplay | null => {
			const message = messages.value.get(id)
			if (!message) {
				console.warn(`Message with id ${id} not found`)
				return null
			}
			return {
				...message,
				text: computed(() => messages.value.get(id)!.text ?? ''),
				over: true,
				hasNext: hasNext,
				hasPrevious: hasPrevious,
			}
		}

		let node = rootMessageId.value
		let displayedRootNode = getNode(node, false, false)
		if(!displayedRootNode) return;
		messagesLocal.push(displayedRootNode)

		for (const decision of fullDecisions) {
			const children = threadTree.value.get(node) ?? []
			if (children.length > 0) {
				if (!children[decision]) break;
				node = children[decision]
				const displayedNode = getNode(node, decision < children.length - 1, decision > 0)
				if(!displayedNode) break;
				messagesLocal.push(displayedNode)
			}
		}

		displayedMessages.value = messagesLocal;
		console.timeEnd(timingIdentifier)
	})

	const loadThreadTree = async (conversationId: string) => {
		return new Promise<void>((resolve, reject) => {
			Commands.getThreadTree(conversationId)
				.then((t) => {
					const newMap = new Map<string, readonly string[]>()
					t.forEach((item) => {
						newMap.set(item.key, Object.freeze(item.children))
					})
					rootMessageId.value = t.find((item) => item.parent === null)?.key ?? null
					threadTree.value = newMap
					console.log("[ChatStore] Thread tree loaded successfully.", { conversationId })
					resolve()
				})
				.catch((e) => {
					console.error("[ChatStore] Fail to load the thread tree", e, { conversationId })
					reject(e)
				})
		})
	}

	const rewriteThreadTreeDecision = (decision: number[]) => {
		if (!rootMessageId.value) return;
		threadTreeDecisions.value = getDefaultThreadTreeDecisions(rootMessageId.value, decision)
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
				console.error('[ChatStore] Failed to load messages:', err, { conversationId })
				reject(err)
			})
		})
	}

	const useDecisionToGetLastMessageId = (threadTree: Map<string, readonly string[]>, rootMessageId: string) => {
		let node = rootMessageId

		for (const decision of threadTreeDecisions.value) {
			const children = threadTree.get(node) ?? []
			if (children.length > 0) {
				node = children[decision]
			}
			else {
				break
			}
		}
		return node
	}

	const loadConversation = async (conversationId: string) => {
		try {
			const identifier = '[ChatStore] Time to load conversation'
			console.time(identifier)
			await loadMessages(conversationId)
			await loadThreadTree(conversationId)

			// root message has been set in loadThreadTree
			threadTreeDecisions.value = getDefaultThreadTreeDecisions(rootMessageId.value!)
			lastMessageId.value = useDecisionToGetLastMessageId(threadTree.value, rootMessageId.value!)
			console.timeEnd(identifier)
		}
		catch (err) {
			console.error('[ChatStore] Failed to load conversation:', err, { conversationId })
		}
	}

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
		parentId = parentId ?? (lastMessageId.value ?? undefined)
		return new Promise<string>((resolve, reject) => {
			const conversationId = currentConversationId.value
			if (!conversationId) {
				console.error('[ChatStore] No conversation selected')
				return
			}
			Commands.addMessage(conversationId, message.text, message.sender, parentId)
				.then(async (id) => {
					messages.value.set(id, { ...message, id })
					lastMessageId.value = id
					threadTree.value.set(id, [])

					if (parentId) threadTree.value.set(parentId, [...(threadTree.value.get(parentId!) ?? []), id])
					threadTreeDecisions.value = getDefaultThreadTreeDecisions(rootMessageId.value!)

					console.log('[ChatStore] Message added successfully:', { id, parentId })
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

					threadTree.value.delete(id)
					threadTree.value.forEach((value, key) => {
						if (value.includes(id)) {
							threadTree.value.set(key, value.filter((v) => v !== id))
						}
					})

					if (rootMessageId.value) threadTreeDecisions.value = getDefaultThreadTreeDecisions(rootMessageId.value)
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
		lastMessageId,
		rootMessageId,

		changeThreadTreeDecision,
		rewriteThreadTreeDecision,
		threadTreeDecisions,
		displayedMessage: displayedMessages,
		loadConversation,
	}
});

