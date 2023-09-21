use crate::api::API_ROOT;
use common::model::discovery::{Movie, RandomMoviesResponse};
use gloo::console::console;
use reqwasm::http;
use std::error::Error;

pub async fn api_get_discovery_movies_random(count: Option<i16>) -> Result<Vec<Movie>, String> {
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

    let title_count = count.unwrap_or_else(|| 25);
    let response =
        match http::Request::get(&*format!("{API_ROOT}/discovery/movies/rand/{title_count}"))
            .credentials(http::RequestCredentials::Include)
            .send()
            .await
        {
            Ok(res) => res,
            Err(_) => return Err("Failed to make request".to_string()),
        };

    let res_json = response.json::<RandomMoviesResponse>().await;

    match res_json {
        Ok(res) => Ok(res.data.movies),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err(format!("Failed to parse API response: {e}"))
        }
    }
}
