use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub trait Model {
    fn collection_name() -> &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
    title: String,
    author: String,
    year: i32,
}

impl Model for Book {
    fn collection_name() -> &'static str {
        "books"
    }
}
