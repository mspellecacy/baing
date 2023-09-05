use actix_web::{get, HttpResponse, patch, Responder, web};
use crate::jwt_auth;

#[get("/collections")]
async fn get_user_collections_handler(jwt_guard: jwt_auth::JwtMiddleware) -> impl Responder {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "collections": "FARTS!"
        })
    });

    HttpResponse::Ok().json(json_response)
}

#[patch("/collections")]
async fn patch_user_collections_handler(jwt_guard: jwt_auth::JwtMiddleware) -> impl Responder {
    HttpResponse::NotImplemented().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(get_user_collections_handler)
        .service(patch_user_collections_handler);
}
