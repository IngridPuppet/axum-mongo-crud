use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

pub trait Model: Sized + Serialize {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub year: i32,
}

impl Book {
    pub fn to_bson(&self) -> Document {
        bson::to_document(self).unwrap()
    }
}

impl Model for Book {}
