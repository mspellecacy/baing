use crate::response::FilteredUser;
use crate::{jwt_auth, AppState};
use actix_web::{get, patch, web, HttpResponse, Responder};
use common::model::user::UpdateUserSchema;
use serde_json::json;

#[get("/user/me")]
async fn get_me_handler(jwt_guard: jwt_auth::JwtMiddleware) -> impl Responder {
    let filtered_user = FilteredUser::from(jwt_guard.user);
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filtered_user
        })
    });

    HttpResponse::Ok().json(json_response)
}

#[patch("/user/me")]
async fn patch_me_handler(
    jwt_guard: jwt_auth::JwtMiddleware,
    body: web::Json<UpdateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user = sqlx::query!(
        r#"UPDATE users SET name = $1, tmdb_api_key = $2 WHERE id = $3 RETURNING name, tmdb_api_key"#,
        body.name.to_owned(),
        body.tmdb_api_key.to_owned(),
        jwt_guard.user.id
    )
    .fetch_one(&data.db)
    .await;

    if let Ok(user) = user {
        let res = json!({
            "status": "success",
            "data":
            {
                "name": user.name,
                "tmdb_api_key": user.tmdb_api_key
            }
        });

        HttpResponse::Ok().json(res)
    } else {
        HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Update failed."
        }))
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(get_me_handler).service(patch_me_handler);
}
