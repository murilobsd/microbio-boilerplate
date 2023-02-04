use anyhow::Result;
use async_trait::async_trait;

use super::identity::Identity;

#[async_trait]
pub trait Repository<I: Identity, T> {
    async fn get_by_id(&self, id: &I) -> Result<T>;
    async fn delete(&self, id: &I) -> Result<()>;
    async fn create(&self, entity: T) -> Result<T>;
}
