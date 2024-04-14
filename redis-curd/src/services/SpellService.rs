use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::error::Error;
use crate::domain::models::Spell::{CreateSpell, Spell};
use crate::domain::repositories::SpellRepository::SpellRepository;
use crate::domain::services::SpellService::SpellService;

pub struct SpellServiceImpl {
    pub repository: Arc<dyn SpellRepository>,
}
impl SpellServiceImpl {
    pub fn new(repository: Arc<dyn SpellRepository>) -> Self {
        SpellServiceImpl { repository }
    }
}
#[async_trait]
impl SpellService for SpellServiceImpl {
    async fn CreateSpell(&self, NewSpell: CreateSpell) -> Result<Spell, Error> {
        let mut cloned = NewSpell.clone();
        self.repository.CreateSpell(&mut cloned).await.map_err(|e| -> Error { e.into() })
    }
    async fn GetAllSpells(&self) -> Result<Vec<Spell>, Error> {
        self.repository.GetAllSpells().await.map_err(|e| -> Error { e.into() })
    }
    async fn GetSpell(&self, SpellID: i32) -> Result<Spell, Error> {
        self.repository.GetSpell(SpellID).await.map_err(|e| -> Error { e.into() })
    }
    async fn RemoveSpell(&self, SpellID: i32) -> Result<(), Error> {
        self.repository.RemoveSpell(SpellID).await.map_err(|e| -> Error { e.into() })
    }
}