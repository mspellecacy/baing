use crate::api::API_ROOT;
use crate::components::header::Header;
use common::model::discovery::{Movie, RandomMoviesResponse};
use common::model::tmdb::{MovieSearch, MovieSearchResult};
use gloo::console::console;
use reqwasm::http;
use reqwasm::http::Headers;
use std::error;

const API_BASE: &str = "https://api.themoviedb.org/3";

pub async fn api_tmdb_get_search_movie_details(
    key: String,
    name: String,
    year: i32,
) -> Result<Vec<MovieSearchResult>, Box<dyn error::Error>> {
    let api_call = format!("/search/movie?query={name}&primary_release_year={year}&include_adult=false&language=en-US&api_key={key}");
    let response = http::Request::get(&*format!("{API_BASE}{api_call}"))
        //.header("Authorization", &*format!("Bearer {key}"))
        .header("accept", "application/json")
        .send()
        .await?;

    let res_json = response.json::<MovieSearch>().await?;

    Ok(res_json.results)
}
