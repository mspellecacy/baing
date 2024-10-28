use allms::llm::{AnthropicModels, OpenAIModels};

pub mod ai_movie;
pub mod ai_tv;
pub mod ai_youtube;

enum AiProviders {
    ANTHROPIC,
    OPENAI,
}
const DEFAULT_PROVIDER: AiProviders = AiProviders::ANTHROPIC;
const OPENAI_MODEL: OpenAIModels = OpenAIModels::Gpt4oMini;
const ANTHROPIC_MODEL: AnthropicModels = AnthropicModels::Claude3_5Sonnet;
