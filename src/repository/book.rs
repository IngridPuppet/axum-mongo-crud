use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    error::Error as MongoError,
    Collection, Database,
};

use super::{Repository, RepositoryError};
use crate::model::{Book, Model};

pub struct MongoBookRepository {
    collection: Collection<Book>,
}

impl MongoBookRepository {
    pub fn from_db(db: &Database) -> Self {
        Self {
            collection: db.collection("books"),
        }
    }
}

#[async_trait]
impl Repository<Book, ObjectId> for MongoBookRepository {

    async fn find_all(&self) -> Result<Vec<Book>, RepositoryError> {
        let mut books: Vec<Book> = vec![];

        // Retrieve all books from the database
        let mut cursor = self.collection.find(None, None).await?;
        while cursor.advance().await? {
            books.push(cursor.deserialize_current()?);
        }

        Ok(books)
    }

    async fn find_one(&self, book_id: ObjectId) -> Result<Option<Book>, RepositoryError> {
        // Query the database for the specified book ID
        Ok(self
            .collection
            .find_one(doc! {"_id": book_id}, None)
            .await?)
    }

    async fn store(&self, book: Book) -> Result<Book, RepositoryError> {
        // Insert the new book into the database
        let metadata = self.collection.insert_one(book.clone(), None).await?;

        // Return persisted book
        Ok(match metadata.inserted_id {
            Bson::ObjectId(oid) => Book {
                id: Some(oid),
                ..book
            },
            _ => unreachable!(),
        })
    }

    async fn update(&self, book: Book) -> Result<Book, RepositoryError> {
        if book.id.is_none() {
            return Err(RepositoryError::MissingIdentifier);
        }

        // Update the book in the database
        let metadata = self
            .collection
            .update_one(
                doc! {"_id": book.id.unwrap().clone()},
                doc! {"$set": book.to_bson()},
                None,
            )
            .await?;

        if metadata.matched_count > 0 {
            Ok(book)
        } else {
            Err(RepositoryError::TargetNotFound)
        }
    }

    async fn delete_one(
        &self,
        book_id: ObjectId,
    ) -> Result<(), RepositoryError> {
        // Delete the book from the database
        let metadata = self
            .collection
            .delete_one(doc! {"_id": book_id}, None)
            .await?;

        if metadata.deleted_count > 0 {
            Ok(())
        } else {
            Err(RepositoryError::TargetNotFound)
        }
    }
}

impl From<MongoError> for RepositoryError {
    fn from(error: MongoError) -> Self {
        RepositoryError::Generic(error.to_string())
    }
}
