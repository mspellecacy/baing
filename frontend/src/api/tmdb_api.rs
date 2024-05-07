use common::model::core::{Movie, MovieDetails, TvShow, TvShowDetails};
use common::model::tmdb::{MovieSearch, TvSearch};
use reqwasm::http;
use std::error;

const API_BASE: &str = "https://api.themoviedb.org/3";

pub async fn api_tmdb_get_search_movie_details(
    key: &str,
    movie: &mut Movie,
) -> Result<Movie, Box<dyn error::Error>> {
    let name = movie.name.as_str();
    let year = movie.year;

    let api_call = format!("/search/movie?query={name}&primary_release_year={year}&include_adult=false&language=en-US&api_key={key}");
    let response = http::Request::get(&format!("{API_BASE}{api_call}"))
        //.header("Authorization", &*format!("Bearer {key}"))
        .header("accept", "application/json")
        .send()
        .await?;

    let res_json = response.json::<MovieSearch>().await?;

    // For now we just pop the first result off the top.
    if let Some(first) = res_json.results.first() {
        let details = MovieDetails::from(first.to_owned());
        movie.details = Some(details)
    }

    Ok(movie.to_owned())
}

pub async fn api_tmdb_get_search_tv_show_details(
    key: &str,
    tv_show: &mut TvShow,
) -> Result<TvShow, Box<dyn error::Error>> {
    let name = tv_show.name.as_str();
    let air_date = tv_show.first_air_date.as_str();

    let api_call = format!("/search/tv?query={name}&first_air_date_year={air_date}&include_adult=false&language=en-US&api_key={key}");
    let response = http::Request::get(&format!("{API_BASE}{api_call}"))
        //.header("Authorization", &*format!("Bearer {key}"))
        .header("accept", "application/json")
        .send()
        .await?;

    let res_json = response.json::<TvSearch>().await?;

    // For now we just pop the first result off the top.
    if let Some(first) = res_json.results.first() {
        let details = TvShowDetails::from(first.to_owned());
        tv_show.details = Some(details)
    }

    Ok(tv_show.to_owned())
}
