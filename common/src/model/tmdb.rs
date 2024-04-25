use serde::{Deserialize, Serialize};
use crate::model::core::DiscoveryMeta;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieSearch {
    pub page: i64,
    pub results: Vec<MovieSearchResult>,
    pub total_pages: i64,
    pub total_results: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieSearchResult {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub id: i64,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvSearch {
    pub page: i64,
    pub results: Vec<TvSearchResult>,
    pub total_pages: i64,
    pub total_results: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvSearchResult {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub id: i64,
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
    pub baing_meta: Option<DiscoveryMeta>,
}
