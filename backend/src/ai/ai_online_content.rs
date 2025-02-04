use crate::ai::{get_typed_special_collections, get_with_instructions, AiProviders, ANTHROPIC_MODEL, DEFAULT_PROVIDER, OPENAI_MODEL};
use crate::ApiKeys;
use allms::Completions;
use common::model::collections::{Media, UserCollection};
use common::model::core::{OnlineContent, TvShow, YTChannel};
use common::model::discovery::{RandomOnlineContentResponseData, RandomTvShowsResponseData, RandomYTChannelsResponseData};
use std::error;
use log::debug;

pub async fn get_random(
    api_keys: &ApiKeys,
    count: i16,
    special_collections: Vec<UserCollection>,
) -> Result<RandomOnlineContentResponseData, Box<dyn error::Error>> {
    // Dummy Media to filter against.
    let media_type = Media::OnlineContent(OnlineContent::default());
    let sp_collections = get_typed_special_collections(media_type, special_collections).await;

    let main_prompt = "You are bAIng, an AI assistant that helps create curated lists of Online Content. You respond only with JSON.";
    let message = format!("\
    Return a diverse collections of {count} Internet Content in form of Podcasts, Blogs, Websites, Twitch Streamers, YouTube Channels, and Other online-first content in the form a JSON Array named \
    'yt_channels' with the fields \
        'name' containing the name of the Online Content as a string, \
        'description' containing a brief description of the YouTube Channel as a String, \
        'url' a link to the content as a String, \
        'language' the country of the Content's language as a i18n-locale String, \
        'bgimage' a link to an image to use for the content or null,
        'tags' a comma seperated list of descriptive tags for this content, \
         and 'baing_meta' containing an object with the sub fields \
            'reason' containing the reason this title was chosen, \
            'query' containing a copy of the original user prompt, \
            and 'streamers' that should be left as an empty string.\
    ");
    let instructions = format!("{} {}", main_prompt, message);
    debug!("AI Instructions: {instructions}");

    Ok(get_with_instructions(api_keys, &instructions).await?)
}


pub async fn get_guided(
    api_keys: &ApiKeys,
    count: i16,
    special_collections: Vec<UserCollection>,
    prompt: &str,
)  -> Result<RandomOnlineContentResponseData, Box<dyn error::Error>> {
    // Dummy Media to filter against.
    let media_type = Media::OnlineContent(OnlineContent::default());
    let sp_collections = get_typed_special_collections(media_type, special_collections).await;

    let main_prompt = "You are bAIng, an AI assistant that helps create curated lists of Online Content. You respond only with JSON.";
    let message = format!("\
    Return a diverse collections of {count} Internet Content in form of Podcasts, Blogs, Websites, Twitch Streamers, YouTube Channels, and Other online-first content based on User's Prompt in the form a JSON Array named \
    'yt_channels' with the fields \
        'name' containing the name of the Online Content as a string, \
        'description' containing a brief description of the YouTube Channel as a String, \
        'url' a link to the content as a String, \
        'language' the country of the Content's language as a i18n-locale String, \
        'bgimage' a link to an image to use for the content or null,
        'tags' a comma seperated list of descriptive tags for this content, \
         and 'baing_meta' containing an object with the sub fields \
            'reason' containing the reason this title was chosen, \
            'query' containing a copy of the original user prompt, \
            and 'streamers' that should be left as an empty string. \n \
        User's Prompt: {prompt} \n \
    ");
    let instructions = format!("{} {}", main_prompt, message);
    debug!("AI Instructions: {instructions}");

    Ok(get_with_instructions(api_keys, &instructions).await?)
}