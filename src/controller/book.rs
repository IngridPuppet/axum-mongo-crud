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

// Fetch all books
pub async fn fetch_all(State(db): State<Database>) -> (StatusCode, Json<Value>) {
    let mut books: Vec<Book> = vec![];

    // Retrieve all books from the database
    let mut cursor = Book::collection(&db).find(None, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        books.push(cursor.deserialize_current().unwrap());
    }

    // Return the books as JSON with an HTTP 200 OK status
    (StatusCode::OK, Json(json!(books)))
}

// Fetch a single book by its ID
pub async fn fetch_one(
    State(db): State<Database>,
    Path(book_id): Path<ObjectId>,
) -> (StatusCode, Json<Value>) {
    // Query the database for the specified book ID
    let cursor = Book::collection(&db)
        .find_one(doc! {"_id": book_id}, None)
        .await
        .unwrap();

    // Check if the book was found and return the appropriate status and JSON
    match cursor {
        Some(book) => (StatusCode::OK, Json(json!(book))),
        None => (StatusCode::NOT_FOUND, Json(Value::Null)),
    }
}

// Store (create) a new book
pub async fn store(
    State(db): State<Database>,
    Json(book): Json<Book>,
) -> (StatusCode, Json<Value>) {
    // Insert the new book into the database
    let res = Book::collection(&db).insert_one(book.clone(), None).await;

    // Check the result and return the appropriate status and JSON
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
            // Print the error and return an HTTP 500 Internal Server Error status
            eprintln!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

// Update an existing book by its ID
pub async fn update(
    State(db): State<Database>,
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
    let res = Book::collection(&db)
        .update_one(doc! {"_id": book_id}, doc! {"$set": book.to_bson()}, None)
        .await;

    // Check the result and return the appropriate status and JSON
    match res {
        Ok(metadata) => {
            if metadata.modified_count > 0 {
                // Book was successfully updated, return an HTTP 200 OK status
                (StatusCode::OK, Json(json!(book)))
            } else {
                // Book was not found, return an HTTP 404 Not Found status
                (StatusCode::NOT_FOUND, Json(Value::Null))
            }
        }
        Err(err) => {
            // Print the error and return an HTTP 500 Internal Server Error status
            eprintln!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

// Delete a book by its ID
pub async fn delete(State(db): State<Database>, Path(book_id): Path<ObjectId>) -> StatusCode {
    // Delete the book from the database
    let deleted_count = Book::collection(&db)
        .delete_one(doc! {"_id": book_id}, None)
        .await
        .unwrap()
        .deleted_count;

    // Check if a book was deleted and return the appropriate HTTP status
    if deleted_count > 0 {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
