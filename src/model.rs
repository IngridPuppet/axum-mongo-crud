use mongodb::{
    bson::{self, oid::ObjectId, Document},
    Collection, Database,
};
use serde::{Deserialize, Serialize};

pub trait Model: Sized + Serialize {
    fn collection(db: &Database) -> Collection<Self>;

    fn to_bson(&self) -> Document {
        bson::to_document(self).unwrap()
    }
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
