use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::bson::oid::ObjectId;
use serde_json::{json, Value};

use crate::{model::Book, repository::RepositoryError, AppState};

// Fetch all books
pub async fn fetch_all(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.book_repository.find_all().await {
        Ok(books) => (StatusCode::OK, Json(json!(books))),
        Err(err) => {
            eprintln!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

// Fetch a single book by its ID
pub async fn fetch_one(
    State(state): State<AppState>,
    Path(book_id): Path<ObjectId>,
) -> (StatusCode, Json<Value>) {
    match state.book_repository.find_one(book_id).await {
        Ok(book) => match book {
            Some(book) => (StatusCode::OK, Json(json!(book))),
            None => (StatusCode::NOT_FOUND, Json(Value::Null)),
        },
        Err(err) => {
            eprintln!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

// Store (create) a new book
pub async fn store(
    State(state): State<AppState>,
    Json(book): Json<Book>,
) -> (StatusCode, Json<Value>) {
    match state.book_repository.store(book).await {
        Ok(book) => (StatusCode::OK, Json(json!(book))),
        Err(err) => {
            eprintln!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

// Update an existing book by its ID
pub async fn update(
    State(state): State<AppState>,
    Path(book_id): Path<ObjectId>,
    Json(book): Json<Book>,
) -> (StatusCode, Json<Value>) {
    // Check if the provided book ID matches the one in the payload
    if Some(book_id) != book.id {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(String::from("resource id mismatch"))),
        );
    }

    // Update the book in the database
    match state.book_repository.update(book).await {
        Ok(book) => (StatusCode::OK, Json(json!(book))),
        Err(err) => match err {
            RepositoryError::TargetNotFound => (StatusCode::NOT_FOUND, Json(Value::Null)),
            err => {
                eprintln!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
            }
        },
    }
}

// Delete a book by its ID
pub async fn delete(
    State(state): State<AppState>,
    Path(book_id): Path<ObjectId>,
) -> StatusCode {
    match state.book_repository.delete_one(book_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => match err {
            RepositoryError::TargetNotFound => StatusCode::NOT_FOUND,
            err => {
                eprintln!("{}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
    }
}
