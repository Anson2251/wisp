use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MultimodalCapability {
    Vision(i32),
    Audio,
    Video,
    Text(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelCategory {
    Chat,
    ImageGeneration,
	Audio,
	TextToSpeech,
    Embed,
	Rerank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelCapability {
    FIM,
    ToolUse,
	Reasoning
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelParamItem {
    Temperature(f32),
    TopP(f32),
    TopK(u32),
    MaxTokens(usize),
    PresencePenalty(f32),
    FrequencyPenalty(f32),
    StopSequences(Vec<String>),
    Seed(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
	pub display_name: String,
    pub parameters: Vec<ModelParamItem>,
    pub category: ModelCategory,
    pub capabilities: Vec<ModelCapability>,
    pub multimodal: Vec<MultimodalCapability>,
}
