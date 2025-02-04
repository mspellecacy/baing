use crate::ApiKeys;
use allms::llm::{AnthropicModels, GoogleModels, OpenAIModels};
use allms::Completions;
use common::model::collections::{Media, UserCollection};
use std::collections::HashMap;
use std::error;

pub mod ai_movie;
pub mod ai_online_content;
pub mod ai_tv;
pub mod ai_youtube;

enum AiProviders {
    ANTHROPIC,
    OPENAI,
}
const DEFAULT_PROVIDER: AiProviders = AiProviders::ANTHROPIC;
const OPENAI_MODEL: OpenAIModels = OpenAIModels::Gpt4oMini;
const ANTHROPIC_MODEL: AnthropicModels = AnthropicModels::Claude3_5Sonnet;
const GOOGLE_MODEL: GoogleModels = GoogleModels::Gemini1_5Pro;

pub async fn get_typed_special_collections(
    media: Media,
    collections: Vec<UserCollection>,
) -> HashMap<String, String> {
    let mut colls = HashMap::new();

    colls.insert(
        String::from("thumbsup"),
        common::model::collections::extract_special_collection_to_entries(
            &collections,
            "thumbsup",
            &media,
        ),
    );

    colls.insert(
        String::from("thumbsdown"),
        common::model::collections::extract_special_collection_to_entries(
            &collections,
            "thumbsdown",
            &media,
        ),
    );

    colls.insert(
        String::from("skipped"),
        common::model::collections::extract_special_collection_to_entries(
            &collections,
            "skipped",
            &media,
        ),
    );

    colls
}

pub async fn get_with_instructions<T>(
    api_keys: &ApiKeys,
    instructions: &str,
) -> Result<T, Box<dyn error::Error>>
where
    T: serde::de::DeserializeOwned + schemars::JsonSchema,
{
    let comp = match DEFAULT_PROVIDER {
        AiProviders::ANTHROPIC => {
            let ant_comps = Completions::new(
                ANTHROPIC_MODEL,
                api_keys
                    .anthropic
                    .clone()
                    .expect("Missing Anthropic API Key")
                    .as_str(),
                None,
                None,
            );
            ant_comps.get_answer::<T>(instructions).await?
        }
        AiProviders::OPENAI => {
            let openai_completions = Completions::new(
                OPENAI_MODEL,
                api_keys
                    .openai
                    .clone()
                    .expect("Missing OpenAI API Key")
                    .as_str(),
                None,
                None,
            )
            .function_calling(false);
            openai_completions.get_answer::<T>(instructions).await?
        }
    };

    Ok(comp)
}
