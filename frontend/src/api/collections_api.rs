use reqwasm::http;
use common::model::user::ErrorResponse;
use crate::api::API_ROOT;

pub async fn api_get_user_collections() -> Result<String, String> {
    let response = match http::Request::get(&*format!("{API_ROOT}/collections"))
        .credentials(http::RequestCredentials::Include)
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

    //let res_json = response.json::<UserResponse>().await;
    // match res_json {
    //     Ok(data) => Ok(data.data.user),
    //     Err(_) => Err("Failed to parse response".to_string()),
    // }
    Ok(String::from("Farts"))
}