use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    error::Error as MongoError,
    Collection, Database,
};

use super::{Repository, RepositoryStatus};
use crate::model::{Book, Model};

pub struct BookRepository {
    collection: Collection<Book>,
}

impl BookRepository {
    pub fn from_db(db: &Database) -> Self {
        Self {
            collection: db.collection("books"),
        }
    }
}

#[async_trait]
impl Repository<Book> for BookRepository {
    type RepositoryError = MongoError;

    async fn find_all(&self) -> Result<Vec<Book>, Self::RepositoryError> {
        let mut books: Vec<Book> = vec![];

        // Retrieve all books from the database
        let mut cursor = self.collection.find(None, None).await?;
        while cursor.advance().await? {
            books.push(cursor.deserialize_current()?);
        }

        Ok(books)
    }

    async fn find_one(&self, book_id: ObjectId) -> Result<Option<Book>, Self::RepositoryError> {
        // Query the database for the specified book ID
        Ok(self
            .collection
            .find_one(doc! {"_id": book_id}, None)
            .await?)
    }

    async fn store(&self, book: Book) -> Result<ObjectId, Self::RepositoryError> {
        // Insert the new book into the database
        let metadata = self.collection.insert_one(book.clone(), None).await?;

        // Return persisted book
        Ok(match metadata.inserted_id {
            Bson::ObjectId(oid) => oid,
            _ => unreachable!(),
        })
    }

    async fn update(&self, book: Book) -> Result<RepositoryStatus, Self::RepositoryError> {
        if book.id.is_none() {
            return Err(MongoError::custom(RepositoryStatus::MissingIdentifier));
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

        Ok(RepositoryStatus::Done)
    }

    async fn delete_one(
        &self,
        book_id: ObjectId,
    ) -> Result<RepositoryStatus, Self::RepositoryError> {
        Ok(RepositoryStatus::Done)
    }
}
