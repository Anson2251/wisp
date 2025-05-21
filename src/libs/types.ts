export enum MessageRole {
	User = "user",
	Assistant = "bot",
	System = "system",
}

export type Message = {
	id: string,
	text: string,
	sender: MessageRole,
	timestamp: number,
	tokens?: number,
	embedding?: Uint8Array,
}

export type Conversation = {
	id: string,
	name: string,
	description?: string,
	entry_message_id?: string,
}
