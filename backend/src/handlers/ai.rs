use std::convert::Into;
use crate::ai::{ai_movie, ai_online_content, ai_tv, ai_youtube};
use crate::db_helpers::get_user_special_collections;
use crate::{jwt_auth, AppState};
use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::service;
use log::debug;
use serde::Deserialize;
use serde_json::json;

// Gpt-4, even though told not to, returns json wrapped in markdown ```json ... ```, breaking serde
fn strip_markdown(in_value: String) -> String {
    in_value.replace("```json", "").replace("```", "")
}

#[derive(Debug, Deserialize)]
struct DiscoveryQuery {
    query: String,
}

#[get("/discovery/movies/rand/{count}")]
async fn get_discovery_movies_rand_n(
    jwt_guard: jwt_auth::JwtMiddleware,
    path: web::Path<i16>,
    dq: web::Query<DiscoveryQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let count = path.into_inner();
    let user_special_collections = get_user_special_collections(jwt_guard.user.id, &data)
        .await
        .expect("Missing User's Special Collections?");

    let query_type = match !dq.query.is_empty() {
        false => {
            ai_movie::get_random(&data.api_keys, count, user_special_collections.to_owned()).await
        }
        true => {
            ai_movie::get_guided(
                &data.api_keys,
                count,
                user_special_collections.to_owned(),
                &dq.query,
            )
            .await
        }
    };

    let random_movies = match query_type {
        Ok(mut res) => {
            debug!("{:#?}", &res);
            json!({
                "status": "success",
                "data": res
            })
        }
        Err(err) => {
            json!({
                "status": "error",
                "message": format!("Error: {err}")
            })
        }
    };

    HttpResponse::Ok().json(random_movies)
}

#[get("/discovery/tv-shows/rand/{count}")]
async fn get_discovery_tv_shows_rand_n(
    jwt_guard: jwt_auth::JwtMiddleware,
    path: web::Path<(i16,)>,
    dq: web::Query<DiscoveryQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let count = path.into_inner().0;
    let user_special_collections = get_user_special_collections(jwt_guard.user.id, &data)
        .await
        .expect("Missing User's Special Collections?");
    let query_type = match !dq.query.is_empty() {
        false => {
            ai_tv::get_random(&data.api_keys, count, user_special_collections.to_owned()).await
        }
        true => {
            ai_tv::get_guided(
                &data.api_keys,
                count,
                user_special_collections.to_owned(),
                &dq.query,
            )
            .await
        }
    };

    let random_tv_shows = match query_type {
        Ok(mut res) => {
            debug!("{:#?}", &res);
            json!({
                "status": "success",
                "data": res
            })
        }
        Err(err) => {
            json!({
                "status": "error",
                "message": format!("Error: {err}")
            })
        }
    };

    HttpResponse::Ok().json(random_tv_shows)
}

#[get("/discovery/yt-channels/rand/{count}")]
async fn get_discovery_yt_channels_rand_n(
    jwt_guard: jwt_auth::JwtMiddleware,
    path: web::Path<(i16,)>,
    dq: web::Query<DiscoveryQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let count = path.into_inner().0;
    let user_special_collections = get_user_special_collections(jwt_guard.user.id, &data)
        .await
        .expect("Missing User's Special Collections?");
    let query_type = match !dq.query.is_empty() {
        false => {
            ai_youtube::get_random(&data.api_keys, count, user_special_collections.to_owned()).await
        }
        true => {
            ai_youtube::get_guided(
                &data.api_keys,
                count,
                user_special_collections.to_owned(),
                &dq.query,
            )
            .await
        }
    };

    let random_yt_channels = match query_type {
        Ok(mut res) => {
            debug!("{:#?}", &res);
            json!({
                "status": "success",
                "data": res
            })
        }
        Err(err) => {
            json!({
                "status": "error",
                "message": format!("Error: {err}")
            })
        }
    };

    HttpResponse::Ok().json(random_yt_channels)
}

#[get("/discovery/online-content/rand/{count}")]
async fn get_discovery_online_content_rand_n(
    jwt_guard: jwt_auth::JwtMiddleware,
    path: web::Path<(i16,)>,
    dq: web::Query<DiscoveryQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let count = path.into_inner().0;
    let user_special_collections = get_user_special_collections(jwt_guard.user.id, &data)
        .await
        .expect("Missing User's Special Collections?");
    let query_type = match !dq.query.is_empty() {
        false => {
            ai_online_content::get_random(&data.api_keys, count, user_special_collections.to_owned()).await
        }
        true => {
            ai_online_content::get_guided(
                &data.api_keys,
                count,
                user_special_collections.to_owned(),
                &dq.query,
            )
                .await
        }
    };

    let random_online_content = match query_type {
        Ok(mut res) => {
            debug!("{:#?}", &res);
            json!({
                "status": "success",
                "data": res
            })
        }
        Err(err) => {
            json!({
                "status": "error",
                "message": format!("Error: {err}")
            })
        }
    };

    HttpResponse::Ok().json(random_online_content)
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(get_discovery_movies_rand_n)
        .service(get_discovery_tv_shows_rand_n)
        .service(get_discovery_yt_channels_rand_n)
        .service(get_discovery_online_content_rand_n);
}
