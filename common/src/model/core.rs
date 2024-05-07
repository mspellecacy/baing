use crate::model::tmdb::{MovieSearchResult, TvSearchResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryMeta {
    pub query: String,
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Movie {
    pub name: String,
    pub year: i32,
    pub details: Option<MovieDetails>,
    pub baing_meta: Option<DiscoveryMeta>,
}

impl Default for Movie {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            year: 0,
            details: None,
            baing_meta: None,
        }
    }
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

// Basic copy of the results value, because I prefer these somewhat uncoupled in the long run.
impl From<MovieSearchResult> for MovieDetails {
    fn from(value: MovieSearchResult) -> Self {
        MovieDetails {
            backdrop_path: value.backdrop_path,
            genre_ids: value.genre_ids,
            tmdb_id: value.id,
            original_language: value.original_language,
            original_title: value.original_title,
            overview: value.overview,
            popularity: value.popularity,
            poster_path: value.poster_path,
            release_date: value.release_date,
            title: value.title,
            vote_average: value.vote_average,
            vote_count: value.vote_count,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TvShow {
    pub name: String,
    pub first_air_date: String,
    pub language: String,
    pub details: Option<TvShowDetails>,
    pub baing_meta: Option<DiscoveryMeta>,
}

impl Default for TvShow {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            first_air_date: "".to_string(),
            language: "".to_string(),
            details: None,
            baing_meta: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvShowDetails {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub tmdb_id: i64,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub first_air_date: String,
    pub name: String,
    pub vote_average: f64,
    pub vote_count: i64,
}

// Basic copy of the results value, because I prefer these somewhat uncoupled in the long run.
impl From<TvSearchResult> for TvShowDetails {
    fn from(value: TvSearchResult) -> Self {
        TvShowDetails {
            adult: false,
            backdrop_path: value.backdrop_path,
            genre_ids: value.genre_ids,
            tmdb_id: value.id,
            origin_country: value.origin_country,
            original_language: value.original_language,
            original_name: value.original_name,
            overview: value.overview,
            popularity: value.popularity,
            poster_path: value.poster_path,
            first_air_date: value.first_air_date,
            name: value.name,
            vote_average: value.vote_average,
            vote_count: value.vote_count,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct YTChannel {
    pub name: String,
    pub description: String,
    pub channel_id: String,
    pub language: String,
    pub details: Option<YTChannelDetails>,
    pub baing_meta: Option<DiscoveryMeta>,
}

impl Default for YTChannel {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            description: "".to_string(),
            channel_id: "".to_string(),
            language: "".to_string(),
            details: None,
            baing_meta: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct YTChannelDetails {
    pub backdrop_path: Option<String>,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
}