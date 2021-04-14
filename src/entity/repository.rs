use crate::entity::utxo::{Utxo, UtxoBalance};
use async_trait::async_trait;

#[async_trait]
pub trait Repository: RepoClone + Send + Sync + 'static {
    async fn find_all(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Utxo>, Box<dyn std::error::Error>>;
    async fn find_balance_by_address(
        &self,
        address: &str,
        spent: bool,
    ) -> Result<Option<UtxoBalance>, Box<dyn std::error::Error>>;
    async fn create_utxo(&self, utxo: &Utxo) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait RepoClone {
    fn clone_box(&self) -> Box<dyn Repository + Send + Sync + 'static>;
}

impl<T> RepoClone for T
where
    T: Repository + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn Repository + Send + Sync + 'static> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Repository + Send + Sync + 'static> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
