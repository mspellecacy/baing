use crate::api::API_ROOT;
use common::model::collections::{IsMedia, Media};
use common::model::core::{DiscoveryMeta, TvShow};
use common::model::discovery::{RandomMoviesResponse, RandomOnlineContentsResponse, RandomTvShowsResponse, RandomYTChannelsResponse, RandomYTChannelsResponseData};
use gloo::console::console;
use reqwasm::http;
use std::ops::Div;

pub async fn api_get_discovery_both_random(
    mut count: Option<i16>,
    query: &str,
) -> Result<Vec<Media>, String> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    if let Some(cnt) = count {
        count = Some(cnt.div(2)); // Split the request count 'evenly' between both types.
    }

    let mut out: Vec<Media> = Vec::new();

    // Hurray Fearless Concurrency!
    let (movies, shows) = futures::join!(
        api_get_discovery_movies_random(count, query),
        api_get_discovery_tv_shows_random(count, query)
    );

    if let Ok(mut movies) = movies {
        out.append(&mut movies);
    }
    if let Ok(mut shows) = shows {
        out.append(&mut shows);
    }

    // Mix'em up before pushing them along
    out.shuffle(&mut thread_rng());
    Ok(out)
}

pub async fn api_get_discovery_movies_random(
    count: Option<i16>,
    query: &str,
) -> Result<Vec<Media>, String> {
    // let mock_response =
    // r#"{
    //     "data": {
    //         "movies": [
    //             {
    //                 "name": "Psycho",
    //                 "year": 1960
    //             },
    //             {
    //                 "name": "The Godfather",
    //                 "year": 1972
    //             },
    //             {
    //                 "name": "Star Wars: Episode IV - A New Hope",
    //                 "year": 1977
    //             },
    //             {
    //                 "name": "The Shawshank Redemption",
    //                 "year": 1994
    //             },
    //             {
    //                 "name": "Get Out",
    //                 "year": 2017
    //             }
    //         ]
    //     },
    //     "status": "success"
    // }"#;
    //let res_json:Result<RandomMoviesResponse, _> = serde_json::from_str(mock_response);

    let title_count = count.unwrap_or(25);
    let response = match http::Request::get(&format!(
        "{}/discovery/movies/rand/{}?query={}",
        API_ROOT, title_count, query
    ))
    .credentials(http::RequestCredentials::Include)
    .send()
    .await
    {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to make request: {e}")),
    };

    let res_json = response.json::<RandomMoviesResponse>().await;

    match res_json {
        Ok(res) => Ok(res.data.movies.into_iter().map(|c| c.as_media()).collect()),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err(format!("Failed to parse API response: {e}"))
        }
    }
}

pub async fn api_get_discovery_tv_shows_random(
    count: Option<i16>,
    query: &str,
) -> Result<Vec<Media>, String> {
    // let mock_response =
    // r#"{
    //     "data":{
    //         "tv_shows":[
    //         {
    //             "first_air_date":"1994-09-22",
    //             "name":"Friends",
    //             "language":"en-US"
    //         },
    //         {
    //             "first_air_date":"2008-01-20",
    //             "name":"Breaking Bad",
    //             "language":"en-US"
    //         },
    //         {
    //             "first_air_date":"2011-04-17",
    //             "name":"Game of Thrones",
    //             "language":"en-US"
    //         },
    //         {
    //             "first_air_date":"1989-12-17",
    //             "name":"The Simpsons",
    //             "language":"en-US"
    //         },
    //         {
    //             "first_air_date":"2010-07-25",
    //             "name":"Sherlock",
    //             "language":"en-GB"
    //         }
    //         ]
    //     }
    //     "status": "success"
    // }"#;
    //
    // let res_json:Result<RandomTvShowsResponse, _> = serde_json::from_str(mock_response);

    // let mut baing_meta = DiscoveryMeta {
    //     discovery_query: query.to_string(),
    //     discovery_reason: "".to_string(),
    // }
    let title_count = count.unwrap_or(25);
    let response = match http::Request::get(&format!(
        "{}/discovery/tv-shows/rand/{}?query={}",
        API_ROOT, title_count, query
    ))
    .credentials(http::RequestCredentials::Include)
    .send()
    .await
    {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to make request: {e}")),
    };

    let res_json = response.json::<RandomTvShowsResponse>().await;

    match res_json {
        Ok(res) => Ok(res
            .data
            .tv_shows
            .into_iter()
            .map(|c| c.as_media())
            .collect()),
        //Ok(res) => Ok(res.data.tv_shows.clone()),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err(format!("Failed to parse API response: {e}"))
        }
    }
}

pub async fn api_get_discovery_yt_channels_random(
    count: Option<i16>,
    query: &str,
) -> Result<Vec<Media>, String> {
    let mock_response = r#"{
        "status": "success",
        "data": {
            "yt_channels": [
                {
                  "name": "Good Mythical Morning",
                  "channel_id": "UC4PooiX37Pld1T8J5SYT-SQ",
                  "description": "Join Rhett and Link for mythical adventures in this comedic variety show filled with bizarre challenges, food experiments, and comedic sketches.",
                  "language": "en-US",
                  "baing_meta": {
                    "reason": "Offers diverse content suitable for watching with family, including short-format episodes.",
                    "query": "Channels that I can watch with my mother that have shorter 15 minute content."
                  }
                },
                {
                  "name": "BuzzFeedVideo",
                  "channel_id": "UCpko_-a4wgz2u_DgDgd9fqA",
                  "description": "BuzzFeed's flagship channel featuring tasty recipes, funny sketches, challenges, and more, perfect for quick entertainment with family.",
                  "language": "en-US",
                  "baing_meta": {
                    "reason": "Offers a variety of short-format entertainment suitable for watching with family.",
                    "query": "Channels that I can watch with my mother that have shorter 15 minute content."
                  }
                },
                {
                  "name": "Tasty",
                  "channel_id": "UCJFp8uSYCjXOMnkUyb3CQ3Q",
                  "description": "Delicious recipes, food hacks, and cooking challenges presented in short, visually appealing videos perfect for family cooking sessions.",
                  "language": "en-US",
                  "baing_meta": {
                    "reason": "Provides quick, family-friendly cooking content in short-format videos.",
                    "query": "Channels that I can watch with my mother that have shorter 15 minute content."
                  }
              }
            ]
        }
    }"#;

    //let res_json:Result<RandomYTChannelsResponse, _> = serde_json::from_str(mock_response);
    // let mut baing_meta = DiscoveryMeta {
    //     discovery_query: query.to_string(),
    //     discovery_reason: "".to_string(),
    // }

    let title_count = count.unwrap_or(25);
    let response = match http::Request::get(&format!(
        "{}/discovery/yt-channels/rand/{}?query={}",
        API_ROOT, title_count, query
    ))
    .credentials(http::RequestCredentials::Include)
    .send()
    .await
    {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to make request: {e}")),
    };

    let res_json = response.json::<RandomYTChannelsResponse>().await;

    match res_json {
        Ok(res) => Ok(res
            .data
            .yt_channels
            .into_iter()
            .map(|c| c.as_media())
            .collect()),
        //Ok(res) => Ok(res.data.tv_shows.clone()),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err(format!("Failed to parse API response: {e}"))
        }
    }
}


pub async fn api_get_discovery_online_content_random(
    count: Option<i16>,
    query: &str,
) -> Result<Vec<Media>, String> {
    let title_count = count.unwrap_or(25);
    let response = match http::Request::get(&format!(
        "{}/discovery/online-content/rand/{}?query={}",
        API_ROOT, title_count, query
    ))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to make request: {e}")),
    };

    let res_json = response.json::<RandomOnlineContentsResponse>().await;

    match res_json {
        Ok(res) => Ok(res
            .data
            .online_content
            .into_iter()
            .map(|c| c.as_media())
            .collect()),
        //Ok(res) => Ok(res.data.tv_shows.clone()),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err(format!("Failed to parse API response: {e}"))
        }
    }


}