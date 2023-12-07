mod book;

use std::default;

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum RepositoryStatus {
    Done,
    MissingIdentifier,
    TargetNotFound,
}

#[async_trait]
pub trait Repository<Model>: Sized {
    type RepositoryError;

    async fn find_all(&self) -> Result<Vec<Model>, Self::RepositoryError>;
    async fn find_one(&self, model_id: ObjectId) -> Result<Option<Model>, Self::RepositoryError>;

    async fn store(&self, model: Model) -> Result<ObjectId, Self::RepositoryError>;
    async fn update(&self, model: Model) -> Result<RepositoryStatus, Self::RepositoryError>;

    async fn delete_one(&self, model_id: ObjectId) -> Result<RepositoryStatus, Self::RepositoryError>;
}
