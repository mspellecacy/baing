use crate::db_helpers::get_user_special_collections;
use crate::{jwt_auth, AppState};
use actix_web::{get, patch, web, HttpResponse, Responder};
use common::model::collections::UserCollection;
use common::model::collections::UserCollectionPatchResponse;
use uuid::Uuid;

#[get("/collections/special")]
async fn get_user_collections_special_handler(
    jwt_guard: jwt_auth::JwtMiddleware,
    data: web::Data<AppState>,
) -> impl Responder {
    let owner_id = jwt_guard.user.id;
    let collections = get_user_special_collections(owner_id, &data).await;

    let json = match collections {
        Ok(cols) => {
            let json_cols = serde_json::json!(cols);
            serde_json::json!({
                "status":  "success",
                "data": serde_json::json!({
                    "collections": json_cols
                })
            })
        }
        Err(e) => {
            serde_json::json!({
                "status":  "error",
                "message": e.to_string()
            })
        }
    };

    HttpResponse::Ok().json(json)
}

#[get("/collections")]
async fn get_user_collections_handler(
    jwt_guard: jwt_auth::JwtMiddleware,
    data: web::Data<AppState>,
) -> impl Responder {
    let owner_id = jwt_guard.user.id;
    let collections = sqlx::query_as!(
        UserCollection,
        r#"SELECT
        id, owner_id,name, created_at, active, collection, locked, tags, special,
        CAST(sharing as text)
        FROM collections
        WHERE owner_id = $1"#,
        owner_id
    )
    .fetch_all(&data.db)
    .await;

    let json = match collections {
        Ok(cols) => {
            let json_cols = serde_json::json!(cols);
            serde_json::json!({
                "status":  "success",
                "data": serde_json::json!({
                    "collections": json_cols
                })
            })
        }
        Err(e) => {
            serde_json::json!({
                "status":  "error",
                "message": e.to_string()
            })
        }
    };

    HttpResponse::Ok().json(json)
}

#[patch("/collection/{id}")]
async fn patch_user_collection_handler(
    jwt_guard: jwt_auth::JwtMiddleware,
    path: web::Path<Uuid>,
    body: web::Json<UserCollection>,
    data: web::Data<AppState>,
) -> impl Responder {
    let owner_id = jwt_guard.user.id;
    let user_collection = body.clone();
    let collection = user_collection.collection;

    let collection = sqlx::query_as!(
        UserCollection,
        r#"
        UPDATE
            collections
        SET
            collection = $1
        WHERE
            owner_id = $2 AND id = $3
        RETURNING
            id, owner_id,name, created_at, active, collection, locked, tags, special,
            CAST(sharing as text)
        "#,
        serde_json::json!(collection),
        owner_id,
        user_collection.id
    )
    .fetch_one(&data.db)
    .await;

    let json = match collection {
        Ok(rec) => {
            serde_json::json!(UserCollectionPatchResponse {
                status: "success".to_string(),
                data: rec
            })
        }
        Err(e) => {
            serde_json::json!({
                "status": "error",
                "message": format!("Update failed: {}", e)
            })
        }
    };

    HttpResponse::Ok().json(json)
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(get_user_collections_handler)
        .service(patch_user_collection_handler);
}
