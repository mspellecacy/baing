
use crate::api::tmdb_api::{
    api_tmdb_get_search_movie_details, api_tmdb_get_search_tv_show_details,
};
use crate::api::youtube_api::api_yt_channel_details;
use common::model::collections::{IsMedia, Media};
use gloo::console::console;
use std::error;

pub mod collections_api;
pub mod discovery_api;
pub mod tmdb_api;
pub mod user_api;
mod youtube_api;

//TODO: need to replace this with an env var.
const API_ROOT: &str = "http://localhost:8000/api";


#[allow(unreachable_patterns)]
pub async fn get_media_details(key: &str, media: &Media) -> Result<Media, Box<dyn error::Error>> {
    match media {
        Media::Movie(m) => Ok(api_tmdb_get_search_movie_details(key, &mut m.clone())
            .await?
            .as_media()),
        Media::TvShow(t) => Ok(api_tmdb_get_search_tv_show_details(key, &mut t.clone())
            .await?
            .as_media()),
        Media::YTChannel(c) => Ok(c.as_media()),
        Media::OnlineContent(oc) => Ok(oc.as_media()),
        _ => unreachable!("Unsupported Media Type")
    }
}

pub async fn coalesce_media(
    key: &str,
    media: &[Media],
) -> Result<Vec<Media>, Box<dyn error::Error>> {
    let mut out = media.to_owned();

    // Feels a little long-winded but this runs all the coalesce requests in parallel regardless
    // of the Media type and then pairs them back together.
    async fn match_up(
        key: &str,
        index: usize,
        media: &Media,
    ) -> Result<(usize, Media), Box<dyn error::Error>> {
        Ok((index, get_media_details(key, media).await?))
    }

    let media_match_up: Vec<_> = media
        .iter()
        .enumerate()
        .map(move |(i, media)| async move { match_up(key, i, media).await })
        .collect();

    let matches: Vec<Result<(usize, Media), Box<dyn error::Error>>> =
        futures::future::join_all(media_match_up).await;

    matches.iter().for_each(|pairing| match pairing {
        Ok((i, m)) => out[*i] = m.to_owned(),
        Err(e) => {
            console!(format!("Error Fetching Media Details: {e:?}"));
        }
    });

    Ok(out)
}
