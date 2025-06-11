export enum MessageRole {
	User = "user",
	Assistant = "bot",
	System = "system",
}

export type Message = {
	id: string,
	text: string,
	reasoning?: string,
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

export enum ModelCategory {
    Chat = "Chat",
    ImageGeneration = "ImageGeneration",
    Audio = "Audio",
    TextToSpeech = "TextToSpeech",
    Embed = "Embed",
    Rerank = "Rerank"
}

export enum ModelCapability {
    FIM = "FIM",
    ToolUse = "ToolUse",
    Reasoning = "Reasoning"
}

export enum MultimodalCapability {
    Vision = "Vision",
    Audio = "Audio",
    Video = "Video",
    Text = "Text"
}

export interface ModelParams {
    temperature?: number;
    top_p?: number;
    top_k?: number;
    max_tokens?: number;
    presence_penalty?: number;
    frequency_penalty?: number;
    stop_sequences?: string[];
    seed?: number;
}

export interface Model {
    name: string;
    display_name: string;
    parameters: ModelParams;
    category: ModelCategory;
    capabilities: ModelCapability[];
    multimodal: MultimodalCapability[];
}

export interface Provider {
    name: string;
    display_name: string;
    base_url: string;
    models: Model[];
}
