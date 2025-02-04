use crate::ai::{get_typed_special_collections, get_with_instructions, AiProviders, ANTHROPIC_MODEL, DEFAULT_PROVIDER, OPENAI_MODEL};
use crate::ApiKeys;
use allms::{
    llm::{AnthropicModels, GoogleModels, LLMModel, MistralModels, OpenAIModels},
    Completions,
};
use common::model::collections::{Media, UserCollection};
use common::model::core::Movie;
use common::model::discovery::RandomMovieResponseData;
use std::{env, error};
use log::debug;

pub async fn get_random(
    api_keys: &ApiKeys,
    count: i16,
    special_collections: Vec<UserCollection>,
) -> Result<RandomMovieResponseData, Box<dyn error::Error>> {
    // Dummy Media::Movie to filter against. I feel like I'm not understanding something with this.
    let media_type = Media::Movie(Movie::default());
    let sp_collections = get_typed_special_collections(media_type, special_collections).await;
    let main_prompt = "You are bAIng, an AI assistant that helps create curated lists of TV shows and Movies. You respond only with JSON.";
    let message = format!("\
        Return a diverse collections of {count} movies from the past 60 years in the form a JSON Array named \
        'movies' with the fields \
            'name' containing the name of the movie as a string, \
            'year' containing the year of the movie's release as a number, \
            and 'baing_meta' containing an object with the sub fields \
                'reason' containing the reason this title was chosen, \
                'query' containing a copy of the original user prompt, \
                and 'streamers' of streaming platforms, a comma seperated list of streaming providers with a link to the title in parentheses. \n \
        Take the following collections of titles into consideration when making you recommendations but do not include any of them with your final output: \
        Titles they disliked: {unliked_list} \n \
        Titles they liked: {liked_list} \n \
        Titles they skipped: {skipped_list}",
        unliked_list = sp_collections.get("thumbsdown").expect("Missing Special Collection"),
        liked_list = sp_collections.get("thumbsup").expect("Missing Special Collection"),
        skipped_list = sp_collections.get("skipped").expect("Missing Special Collection")
    );
    let instructions = format!("{} {}", main_prompt, message);
    debug!("AI Instructions: {instructions}");

    Ok(get_with_instructions(api_keys, &instructions).await?)
}

pub async fn get_guided(
    api_keys: &ApiKeys,
    count: i16,
    special_collections: Vec<UserCollection>,
    prompt: &str,
) -> Result<RandomMovieResponseData, Box<dyn error::Error>> {
    // Dummy Media::Movie to filter against. I feel like I'm not understanding something with this.
    let media_type = Media::Movie(Movie::default());
    let sp_collections = get_typed_special_collections(media_type, special_collections).await;

    let main_prompt = "You are bAIng, an AI assistant that helps create curated lists of TV shows and Movies. You respond only with JSON.";
    let message = format!("\
        Return a collections of {count} movies based on User's Prompt in the form a JSON Array named \
        'movies' with the fields \
            'name' containing the name of the movie as a string, \
            'year' containing the year of the movie's release as a number, \
            and 'baing_meta' containing an object with the sub fields \
                'reason' containing the reason this title was chosen, \
                'query' containing a copy of the original user prompt, \
                and 'streamers' of streaming platforms with a link to the title in parentheses . \n \
        User's Prompt: {prompt} \n \
        Take the following collections of titles into consideration when making you recommendations but do not include any of them with your final output: \
        Titles they disliked: {unliked_list} \n \
        Titles they liked: {liked_list} \n \
        Titles they skipped: {skipped_list}",
          unliked_list = sp_collections.get("thumbsdown").expect("Missing Special Collection"),
          liked_list = sp_collections.get("thumbsup").expect("Missing Special Collection"),
          skipped_list = sp_collections.get("skipped").expect("Missing Special Collection")
    );
    let instructions = format!("{} {}", main_prompt, message);
    debug!("AI Instructions: {instructions}");

    Ok(get_with_instructions(api_keys, &instructions).await?)
}
