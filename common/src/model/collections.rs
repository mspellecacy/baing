use crate::model::discovery::Movie;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use uuid::Uuid;

// {
// "id": "f6e5d1b9-7492-4624-8857-c1f1c69418c3",
// "owner_id": "a6f40ac4-2b28-48c2-8f3c-be34ce12f17e",
// "name": "Thumbs Up",
// "created_at": "2023-09-05 21:16:29.312364 +00:00",
// "active": true,
// "sharing": "private",
// "collection": [],
// "locked": true,
// "tags": ["Thumbs Up"]
// },

// #[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
// pub enum ShareType {
//     Private,
//     Public,
//     Friends,
// }

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Media {
    Movie(Movie),
    TvShow(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MediaCollection {
    pub entries: Vec<Media>,
}

impl From<Option<JsonValue>> for MediaCollection {
    fn from(value: Option<JsonValue>) -> Self {
        if let Some(json_value) = value {
            match serde_json::from_value(json_value.clone()) {
                Ok(val) => val,
                Err(e) => {
                    println!("json: {}", json_value.to_string());
                    println!("error: {}", e.to_string());

                    MediaCollection { entries: vec![] }
                }
            }
        } else {
            MediaCollection { entries: vec![] }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserCollection {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub active: bool,
    pub sharing: Option<String>,
    pub collection: MediaCollection,
    pub locked: bool,
    pub tags: JsonValue,
    pub special: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserCollectionData {
    pub collections: Vec<UserCollection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCollectionResponse {
    pub status: String,
    pub data: UserCollectionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCollectionPatchResponse {
    pub status: String,
    pub data: UserCollection,
}
