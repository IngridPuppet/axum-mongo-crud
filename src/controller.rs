use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use serde_json::{json, Value};

use crate::model::{Book, Model};

fn collection(db: &Database) -> Collection<Book> {
    db.collection(Book::collection_name())
}

pub async fn fetch_all(State(db): State<Database>) -> (StatusCode, Json<Value>) {
    let mut books: Vec<Book> = vec![];

    let mut cursor = collection(&db).find(None, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        books.push(cursor.deserialize_current().unwrap());
    }

    (StatusCode::OK, Json(json!(books)))
}

pub async fn fetch_one(
    State(db): State<Database>,
    Path(book_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let mongo_id = match ObjectId::from_str(&book_id) {
        Ok(oid) => oid,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err.to_string()
                })),
            )
        }
    };

    let cursor = collection(&db)
        .find_one(doc! {"_id": mongo_id}, None)
        .await
        .unwrap();

    match cursor {
        Some(book) => (StatusCode::OK, Json(json!(book))),
        None => (StatusCode::NOT_FOUND, Json(Value::Null)),
    }
}
