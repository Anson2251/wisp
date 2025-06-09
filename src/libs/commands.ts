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

export async function addMessage(conversationId: string, text: string, sender: string, reasoning?: string, parentId?: string): Promise<string> {
	console.log(reasoning)
	return new Promise((resolve, reject) => {
		invoke('add_message', { conversationId, text, reasoning, sender, parentId })
			.then((messageId) => resolve(messageId as string))
			.catch((error: any) => reject(error));
	});
}

export async function updateMessage(messageId: string, text: string, reasoning?: string): Promise<void> {
	return new Promise<void>((resolve, reject) => {
		invoke('update_message', { messageId, text, reasoning })
			.then(() => resolve())
			.catch((error: any) => reject(error));
	});
}

export async function getMessage(messageId: string): Promise<Message> {
	return new Promise<Message>((resolve, reject) => {
		invoke('get_message', { messageId })
			.then((message) => resolve(message as Message))
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

export async function getAllMessageInvolved(conversationId: string): Promise<Message[]> {
	return new Promise((resolve, reject) => {
		invoke('get_all_message_involved', { conversationId })
			.then((messages) => resolve(messages as Message[]))
			.catch((error: any) => reject(error));
	});
}

type GetThreadTreeResponse = {
	key: string,
	parent: string | null,
	children: string[]
}[]
export async function getThreadTree(conversationId: string): Promise<GetThreadTreeResponse> {
	return new Promise((resolve, reject) => {
		invoke('get_thread_tree', { conversationId })
			.then((threadTree) => resolve(threadTree as GetThreadTreeResponse))
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

export async function updateConversation(conversationId: string, newMetaData: Partial<Omit<Omit<Conversation, 'id'>, 'entry_message_id'>>) {
	return new Promise<void>((resolve, reject) => {
		invoke('update_conversation', { conversationId, ...newMetaData })
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

export interface DiagramCacheEntry {
	svg: string;
	height: number;
	width: number;
}

export async function getCachedDiagram(hash: string): Promise<DiagramCacheEntry | null> {
	return new Promise((resolve, reject) => {
		invoke('get_cached_diagram', { hash })
			.then((entry) => resolve(entry as DiagramCacheEntry | null))
			.catch((error: any) => reject(error));
	});
}

export async function putCachedDiagram(hash: string, entry: DiagramCacheEntry): Promise<void> {
	return new Promise((resolve, reject) => {
		invoke('put_cached_diagram', { hash, entry })
			.then(() => resolve())
			.catch((error: any) => reject(error));
	});
}

export async function clearDiagramCache(): Promise<void> {
    return new Promise((resolve, reject) => {
        invoke('clear_diagram_cache', {})
            .then(() => resolve())
            .catch((error: any) => reject(error));
    });
}

export interface HttpRequest {
  url: string;
  headers?: Record<string, string>;
  parseJson?: boolean;
}

export interface PostRequest extends HttpRequest {
  body: string;
}

export async function getUrl(request: HttpRequest): Promise<any> {
  return invoke('get_url', {
    url: request.url,
    headers: request.headers,
    parseJson: request.parseJson ?? false
  });
}

export async function postUrl(request: PostRequest): Promise<any> {
  return invoke('post_url', {
    url: request.url,
    body: request.body,
    headers: request.headers,
    parseJson: request.parseJson ?? false
  });
}
