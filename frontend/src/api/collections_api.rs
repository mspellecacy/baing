use crate::api::API_ROOT;
use common::model::collections::{
    UserCollection, UserCollectionPatchResponse, UserCollectionResponse,
};
use common::model::user::ErrorResponse;
use gloo::console::console;
use reqwasm::http;

pub async fn api_get_user_collections() -> Result<Vec<UserCollection>, String> {
    let response = match http::Request::get(&format!("{API_ROOT}/collections"))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        return if let Ok(error_response) = error_response {
            Err(error_response.message)
        } else {
            Err(format!("API error: {}", response.status()))
        };
    }

    let res_json = response.json::<UserCollectionResponse>().await;
    match res_json {
        Ok(res) => Ok(res.data.collections),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err("Failed to parse response.".to_string())
        }
    }
}

pub async fn api_patch_user_collection(
    user_collection: UserCollection,
) -> Result<UserCollection, String> {
    let uc_id = user_collection.id;
    let uc_json = serde_json::to_string_pretty(&user_collection)
        .expect("Error Serializing User Collection into JSON payload");

    let response = match http::Request::patch(&format!("{API_ROOT}/collection/{uc_id}"))
        .credentials(http::RequestCredentials::Include)
        .header("Content-Type", "application/json")
        //.header(ACCEPT, "application/json")
        .body(uc_json)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserCollectionPatchResponse>().await;
    match res_json {
        Ok(res) => Ok(res.data),
        Err(e) => {
            console!(format!("Error Parsing Response JSON: {e:?}"));
            Err("Failed to parse response.".to_string())
        }
    }
}
