use super::*;
use entity::repository::Repository;

#[derive(Clone)]
pub struct State {
    db: Box<dyn Repository + Send + Sync + 'static>,
}

impl State {
    pub(crate) async fn new(
        repository: Box<dyn Repository + Send + Sync + 'static>,
    ) -> tide::Result<Self> {
        Ok(Self { db: repository })
    }

    #[allow(clippy::borrowed_box)]
    pub fn db(&self) -> &Box<dyn Repository + Send + Sync + 'static> {
        &self.db
    }
}
