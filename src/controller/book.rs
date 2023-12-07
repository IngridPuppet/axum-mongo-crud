use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    Database,
};
use serde_json::{json, Value};

use crate::model::{Book, Model};

pub async fn fetch_all(State(db): State<Database>) -> (StatusCode, Json<Value>) {
    let mut books: Vec<Book> = vec![];

    let mut cursor = Book::collection(&db).find(None, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        books.push(cursor.deserialize_current().unwrap());
    }

    (StatusCode::OK, Json(json!(books)))
}

pub async fn fetch_one(
    State(db): State<Database>,
    Path(book_id): Path<ObjectId>,
) -> (StatusCode, Json<Value>) {
    let cursor = Book::collection(&db)
        .find_one(doc! {"_id": book_id}, None)
        .await
        .unwrap();

    match cursor {
        Some(book) => (StatusCode::OK, Json(json!(book))),
        None => (StatusCode::NOT_FOUND, Json(Value::Null)),
    }
}

pub async fn store(
    State(db): State<Database>,
    Json(book): Json<Book>,
) -> (StatusCode, Json<Value>) {
    let res = Book::collection(&db).insert_one(book.clone(), None).await;

    match res {
        Ok(metadata) => match metadata.inserted_id {
            Bson::ObjectId(oid) => (
                StatusCode::OK,
                Json(json!(Book {
                    id: Some(oid),
                    ..book
                })),
            ),
            _ => unreachable!(),
        },
        Err(err) => {
            eprintln!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

pub async fn update(
    State(db): State<Database>,
    Path(book_id): Path<ObjectId>,
    Json(book): Json<Book>,
) -> (StatusCode, Json<Value>) {
    if Some(book_id) != book.id {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(String::from("resource id mismatch"))),
        );
    }

    let res = Book::collection(&db)
        .update_one(doc! {"_id": book_id}, doc! {"$set": book.to_bson()}, None)
        .await;

    match res {
        Ok(metadata) => {
            if metadata.modified_count > 0 {
                (StatusCode::OK, Json(json!(book)))
            } else {
                (StatusCode::NOT_FOUND, Json(Value::Null))
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

pub async fn delete(State(db): State<Database>, Path(book_id): Path<ObjectId>) -> StatusCode {
    let deleted_count = Book::collection(&db)
        .delete_one(doc! {"_id": book_id}, None)
        .await
        .unwrap()
        .deleted_count;

    if deleted_count > 0 {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
