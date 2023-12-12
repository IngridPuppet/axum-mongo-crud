pub mod book;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum RepositoryError {
    #[error("generic: {0}")]
    Generic(String),
    #[error("missing identifier")]
    MissingIdentifier,
    #[error("target not found")]
    TargetNotFound,
}

#[async_trait]
pub trait Repository<Model, ModelKeyType>: Sync + Send {
    async fn find_all(&self) -> Result<Vec<Model>, RepositoryError>;
    async fn find_one(&self, model_id: ModelKeyType) -> Result<Option<Model>, RepositoryError>;

    async fn store(&self, model: Model) -> Result<Model, RepositoryError>;
    async fn update(&self, model: Model) -> Result<Model, RepositoryError>;

    async fn delete_one(&self, model_id: ModelKeyType) -> Result<(), RepositoryError>;
}
