import { invoke } from "@tauri-apps/api/core";
import { Message, Conversation } from "./types";

export async function hashContent(content: string): Promise<string> {
	return new Promise((resolve, reject) => {
		invoke('hash_content', { content })
			.then((hash) => resolve(hash as string))
			.catch((error: any) => reject(error));
	});
}


export async function createConversation(name: string, description?: string): Promise<string> {
	return new Promise((resolve, reject) => {
		invoke('create_conversation', { name, description })
			.then((conversationId) => resolve(conversationId as string))
			.catch((error: any) => reject(error));
	});
}

export async function addMessage(conversationId: string, text: string, sender: string, parentId?: string): Promise<string> {
	return new Promise((resolve, reject) => {
		invoke('add_message', { conversationId, text, sender, parentId })
			.then((messageId) => resolve(messageId as string))
			.catch((error: any) => reject(error));
	});
}

export async function updateMessage(messageId: string, text: string): Promise<void> {
	return new Promise<void>((resolve, reject) => {
		invoke('update_message', { messageId, text })
			.then(() => resolve())
			.catch((error: any) => reject(error));
	});
}

export async function deleteMessage(messageId: string, recursive: boolean): Promise<string | null> {
	return new Promise<string | null>((resolve, reject) => {
		invoke('delete_message', { messageId, recursive })
			.then((newParentId) => resolve(newParentId as (string | null)))
			.catch((error: any) => reject(error));
	});
}

export async function getConversationThread(conversationId: string): Promise<Message[]> {
	return new Promise((resolve, reject) => {
		invoke('get_conversation_thread', { conversationId })
			.then((messages) => resolve(messages as Message[]))
			.catch((error: any) => reject(error));
	});
}

export async function updateConversationEntryId(conversationId: string, newEntryId: string): Promise<void> {
	return new Promise((resolve, reject) => {
		invoke('update_conversation_entry_id', { conversationId, messageId: newEntryId })
			.then(() => resolve())
			.catch((error: any) => reject(error));
	});
}

export async function deleteConversation(conversationId: string): Promise<void> {
	return new Promise((resolve, reject) => {
		invoke('delete_conversation', { conversationId })
			.then(() => resolve())
			.catch((error: any) => reject(error));
	});
}

export async function listConversations(): Promise<Conversation[]> {
	return new Promise((resolve, reject) => {
		invoke('list_conversations', {})
			.then((conversations) => resolve(conversations as Conversation[]))
			.catch((error: any) => reject(error));
	});
}
