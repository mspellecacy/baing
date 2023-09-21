use crate::model::tmdb::MovieSearchResult;
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Movie {
    pub name: String,
    pub year: i32,
    pub details: Option<MovieDetails>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MovieDetails {
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub tmdb_id: i64,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: String,
    pub title: String,
    pub vote_average: f64,
    pub vote_count: i64,
}

// Basic copy of the results value, but I prefer these somewhat uncoupled.
impl From<MovieSearchResult> for MovieDetails {
    fn from(value: MovieSearchResult) -> Self {
        MovieDetails {
            backdrop_path: value.backdrop_path,
            genre_ids: value.genre_ids.to_owned(),
            tmdb_id: value.id,
            original_language: value.original_language.to_owned(),
            original_title: value.original_title.to_owned(),
            overview: value.overview.to_owned(),
            popularity: value.popularity.to_owned(),
            poster_path: value.poster_path.to_owned(),
            release_date: value.release_date.to_owned(),
            title: value.title.to_owned(),
            vote_average: value.vote_average.to_owned(),
            vote_count: value.vote_count.to_owned(),
        }
    }
}
