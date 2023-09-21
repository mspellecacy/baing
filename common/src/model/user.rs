use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub tmdb_api_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateResponse {
    pub status: String,
    pub data: UserUpdateData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateData {
    pub name: String,
    pub tmdb_api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserSchema {
    pub name: String,
    pub tmdb_api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
