use crate::response::FilteredUser;
use actix_web::web;
use common::model::user::User;
pub mod ai;
pub mod auth;
pub mod collections;
pub mod user;

fn filter_user_record(user: &User) -> FilteredUser {
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
