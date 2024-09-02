use async_trait::async_trait;
use crate::domain::error::Error;
use crate::domain::models::Spell::{CreateSpell, Spell};

#[async_trait]
pub trait SpellService: Send + Sync {
    async fn CreateSpell(&self, NewSpell: CreateSpell) -> Result<Spell, Error>;
    async fn GetAllSpells(&self) -> Result<Vec<Spell>, Error>;
    async fn GetSpell(&self, SpellID: i32) -> Result<Spell, Error>;
    async fn RemoveSpell(&self, SpellID: i32) -> Result<(), Error>;
}