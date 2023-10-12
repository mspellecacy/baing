use crate::handlers::ai;
use common::model::collections::{Media, UserCollection};
use common::model::core::Movie;
use common::model::*;
use llm_chain::step::Step;
use llm_chain::{chains, parameters, prompt};
use llm_chain_openai::chatgpt::Executor;
use log::debug;
use std::error;

pub async fn get_random_movies(
    exec: &Executor,
    count: i16,
    special_collections: Vec<UserCollection>,
) -> Result<String, Box<dyn error::Error>> {
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
    let message = format!("Return a diverse collections of {count} movies from the past 60 years in the form a JSON Array named 'movies' with the fields 'name' containing the name of the movie as a string, and 'year' containing the year of the movie's release as a number. Take the following collections of titles into consideration when making you recommendations but do not include any of them with your final output: Titles they disliked: {unliked_list} \n Titles they liked: {liked_list} \n Title they skipped: {skipped_list}");
    debug!("{}", &message);

    let chain = chains::conversation::Chain::new(llm_chain::prompt!(system: main_prompt));
    let step1 = Step::for_prompt_template(prompt!(user: message.as_str()));
    let res1 = chain?.send_message(step1, &parameters!(), exec).await?;
    let out = res1.to_immediate().await?;

    Ok(out
        .primary_textual_output()
        .expect("Bad response from OpenAI?"))
}
