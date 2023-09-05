use gloo::console::console;
use reqwasm::http;
use serde_json::json;
use common::model::user::{ErrorResponse, User, UserLoginResponse, UserResponse, UserUpdateData, UserUpdateResponse};
use crate::api::API_ROOT;


pub async fn api_register_user(user_data: &str) -> Result<User, String> {
    let response = match http::Request::post(&*format!("{API_ROOT}/auth/register"))
        .header("Content-Type", "application/json")
        .body(user_data)
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

    let res_json = response.json::<UserResponse>().await;

    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(e) => Err(format!("Failed to parse response: {e:?}")),
    }
}

pub async fn api_login_user(credentials: &str) -> Result<UserLoginResponse, String> {
    let response = match http::Request::post(&*format!("{API_ROOT}/auth/login"))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
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

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to parse response: {e:?}")),
    }
}

pub async fn api_user_info() -> Result<User, String> {
    let response = match http::Request::get(&*format!("{API_ROOT}/user/me"))
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

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(e) => Err(format!("Failed to parse response: {e:?}")),
    }
}

pub async fn api_update_user(user: UserUpdateData) -> Result<UserUpdateData, String> {
    let json_body = json!(user).to_string();
    let response = match http::Request::patch(&*format!("{API_ROOT}/user/me"))
        .credentials(http::RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .body(json_body)
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
    let res_json = response.json::<UserUpdateResponse>().await;
    match res_json {
        Ok(res) => Ok(res.data),
        Err(e) => {
            console!(format!("Error Parsing Update Response JSON: {e:?}"));
            Err(format!("Failed to parse response."))
        },
    }
}

pub async fn api_logout_user() -> Result<(), String> {
    let response = match http::Request::get(&*format!("{API_ROOT}/auth/logout"))
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

    Ok(())
}
