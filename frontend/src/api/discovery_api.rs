use crate::api::API_ROOT;
use common::model::collections::{IsMedia, Media};
use common::model::discovery::{RandomMoviesResponse, RandomTvShowsResponse};
use gloo::console::console;
use reqwasm::http;
use std::ops::Div;
use common::model::core::{DiscoveryMeta, TvShow};

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
        Ok(res) => {
            // for media in &res.data.tv_shows {
            //     match &media.as_media() {
            //         Media::Movie(m) => { console!(format!("{:?}", &m.baing_meta)); }
            //         Media::TvShow(t) => { console!(format!("{:?}", &t.baing_meta)); }
            //     }
            // }

            for show in &res.data.tv_shows {
                console!(format!("{:?}", &show.baing_meta));
            }

            Ok(res
                .data
                .tv_shows
                .into_iter()
                .map(|c| c.as_media())
                .collect())
        },
        //Ok(res) => Ok(res.data.tv_shows.clone()),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err(format!("Failed to parse API response: {e}"))
        }
    }
}
