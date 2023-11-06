use chrono::prelude::*;
use serde::Serialize;
use common::model::user::User;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tmdb_api_key: String,
}

impl From<User> for FilteredUser {
    fn from(user: User) -> Self {
        FilteredUser {
            id: user.id.to_string(),
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            photo: user.photo.to_owned(),
            role: user.role.to_owned(),
            verified: user.verified,
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
            tmdb_api_key: user.tmdb_api_key.to_owned().unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}
