use mongodb::{bson::oid::ObjectId, Collection, Database};
use serde::{Deserialize, Serialize};

pub trait Model {
    fn collection(db: &Database) -> Collection<Self>
    where
        Self: Sized;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub year: i32,
}

impl Model for Book {
    fn collection(db: &Database) -> Collection<Self> {
        db.collection("books")
    }
}
