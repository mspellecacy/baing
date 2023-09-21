use crate::db_helpers::get_user_special_collections;
use crate::{jwt_auth, AppState};
use actix_web::{get, web, HttpResponse, Responder};
use common::model::collections::{Media, UserCollection};
use llm_chain::step::Step;
use llm_chain::{chains, parameters, prompt};
use llm_chain_openai::chatgpt::Executor;
use redis::AsyncCommands;
use serde_json::{json, Value};
use sqlx::Row;
use std::error;

fn extract_special_collection_to_entries(
    special_collection: &Vec<UserCollection>,
    special_name: &str,
) -> String {
    special_collection
        .iter()
        .filter(|uc| uc.special.clone().is_some_and(|s| s == special_name))
        .map(|uc| {
            uc.collection
                .entries
                .iter()
                .map(|media| match media {
                    Media::Movie(movie) => {
                        format!("{} ({})", movie.name.clone(), movie.year) // Interstellar (2014)
                    }
                    Media::TvShow(tv_show) => {
                        format!("//TODO: tv_show")
                    }
                })
                .collect::<Vec<String>>()
                .join(", ") // ".. Jurassic Park (1993), Interstellar (2014) .."
        })
        .collect::<Vec<String>>()
        .join(" ")
}

async fn get_random_movies(
    exec: &Executor,
    count: i16,
    special_collections: Vec<UserCollection>,
) -> Result<String, Box<dyn error::Error>> {
    let unliked_list = extract_special_collection_to_entries(&special_collections, "thumbsdown");
    let liked_list = extract_special_collection_to_entries(&special_collections, "thumbsup");
    let skipped_list = extract_special_collection_to_entries(&special_collections, "skipped");

    let main_prompt = "You are bAIng, an AI assistant that helps create curated lists of TV shows and Movies. You respond only with JSON.";
    let message = format!("Return a diverse collections of {count} movies from the past 60 years in the form a JSON Array named 'movies' with the fields 'name' containing the name of the movie as a string, and 'year' containing the year of the movie's release as a number. Take the following collections of titles into consideration when making you recommendations but do not include any of them with your final output: Titles they disliked: {unliked_list} \n Titles they liked: {liked_list} \n Title they skipped: {skipped_list}");
    // println!("{}", &message);

    let mut chain = chains::conversation::Chain::new(llm_chain::prompt!(system: main_prompt));
    let step1 = Step::for_prompt_template(prompt!(user: message.as_str()));
    let res1 = chain?.send_message(step1, &parameters!(), exec).await?;
    let out = res1.to_immediate().await?;

    Ok(out.primary_textual_output().expect("Bad response from OpenAI?"))
}

#[get("/discovery/movies/rand/{count}")]
async fn get_discovery_movies_rand_n(
    jwt_guard: jwt_auth::JwtMiddleware,
    path: web::Path<(i16,)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let count = path.into_inner().0;
    let user_special_collections = get_user_special_collections(jwt_guard.user.id, &data)
        .await
        .expect("Missing User's Special Collections?");
    let random_movies =
        match get_random_movies(&data.chatgpt, count, user_special_collections).await {
            Ok(res) => {
                let movies = serde_json::from_str::<Value>(res.clone().as_mut_str()).unwrap();

                json!({
                    "status": "success",
                    "data": movies
                })
            }
            Err(err) => {
                json!({
                    "status": "error",
                    "message": format!("Error: {err}")
                })
            }
        };

    HttpResponse::Ok().json(random_movies)
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(get_discovery_movies_rand_n);
}
