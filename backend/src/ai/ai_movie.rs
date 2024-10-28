use crate::ai::{AiProviders, ANTHROPIC_MODEL, DEFAULT_PROVIDER, OPENAI_MODEL};
use crate::ApiKeys;
use allms::{
    llm::{AnthropicModels, GoogleModels, LLMModel, MistralModels, OpenAIModels},
    Completions,
};
use common::model::collections::{Media, UserCollection};
use common::model::core::Movie;
use common::model::discovery::RandomMovieResponseData;
use std::{env, error};

pub async fn get_random(
    api_keys: &ApiKeys,
    count: i16,
    special_collections: Vec<UserCollection>,
) -> Result<RandomMovieResponseData, Box<dyn error::Error>> {
    // Dummy Media::Movie to filter against. I feel like I'm not understanding something with this.
    let media_type = Media::Movie(Movie::default());

    let unliked_list = common::model::collections::extract_special_collection_to_entries(
        &special_collections,
        "thumbsdown",
        &media_type,
    );
    let liked_list = common::model::collections::extract_special_collection_to_entries(
        &special_collections,
        "thumbsup",
        &media_type,
    );
    let skipped_list = common::model::collections::extract_special_collection_to_entries(
        &special_collections,
        "skipped",
        &media_type,
    );

    let main_prompt = "You are bAIng, an AI assistant that helps create curated lists of TV shows and Movies. You respond only with JSON.";
    let message = format!("Return a diverse collections of {count} movies from the past 60 years in the form a JSON Array named 'movies' with the fields 'name' containing the name of the movie as a string, and 'year' containing the year of the movie's release as a number, and 'baing_meta' containing an object with two sub fields 'reason' containing the reason this title was chosen, and 'query' containing a copy of the original user prompt. Take the following collections of titles into consideration when making you recommendations but do not include any of them with your final output: Titles they disliked: {unliked_list} \n Titles they liked: {liked_list} \n Title they skipped: {skipped_list}");
    let instructions = format!("{} {}", main_prompt, message);

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
            ant_comps
                .get_answer::<RandomMovieResponseData>(instructions.as_str())
                .await
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
            openai_completions
                .get_answer::<RandomMovieResponseData>(instructions.as_str())
                .await
        }
    };

    Ok(comp?)
}

pub async fn get_guided(
    api_keys: &ApiKeys,
    count: i16,
    special_collections: Vec<UserCollection>,
    prompt: &str,
) -> Result<RandomMovieResponseData, Box<dyn error::Error>> {
    // Dummy Media::Movie to filter against. I feel like I'm not understanding something with this.
    let media_type = Media::Movie(Movie::default());

    let unliked_list = common::model::collections::extract_special_collection_to_entries(
        &special_collections,
        "thumbsdown",
        &media_type,
    );
    let liked_list = common::model::collections::extract_special_collection_to_entries(
        &special_collections,
        "thumbsup",
        &media_type,
    );
    let skipped_list = common::model::collections::extract_special_collection_to_entries(
        &special_collections,
        "skipped",
        &media_type,
    );

    let main_prompt = "You are bAIng, an AI assistant that helps create curated lists of TV shows and Movies. You respond only with JSON.";
    let message = format!("Return a collections of {count} movies based on User's Prompt in the form a JSON Array named 'movies' with the fields 'name' containing the name of the movie as a string, 'year' containing the year of the movie's release as a number, and 'baing_meta' containing an object with two sub fields 'reason' containing the reason this title was chosen, and 'query' containing a copy of the original user prompt. \n User's Prompt: {prompt} \nTake the following collections of titles into consideration when making you recommendations but do not include any of them with your final output: Titles they disliked: {unliked_list} \n Titles they liked: {liked_list} \n Title they skipped: {skipped_list}");
    let instructions = format!("{} {}", main_prompt, message);

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
            ant_comps
                .get_answer::<RandomMovieResponseData>(instructions.as_str())
                .await
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

            openai_completions
                .get_answer::<RandomMovieResponseData>(instructions.as_str())
                .await
        }
    };

    Ok(comp?)
}
