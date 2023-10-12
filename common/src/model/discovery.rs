use crate::model::core::{Movie, TvShow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RandomMovieResponseData {
    pub movies: Vec<Movie>,
}

#[derive(Serialize, Deserialize)]
pub struct RandomMoviesResponse {
    pub status: String,
    pub data: RandomMovieResponseData,
}

#[derive(Serialize, Deserialize)]
pub struct RandomTvShowsResponseData {
    pub tv_shows: Vec<TvShow>,
}

#[derive(Serialize, Deserialize)]
pub struct RandomTvShowsResponse {
    pub status: String,
    pub data: RandomTvShowsResponseData,
}
