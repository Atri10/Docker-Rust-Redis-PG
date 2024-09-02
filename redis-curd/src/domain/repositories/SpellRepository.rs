use async_trait::async_trait;
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::models::Spell::CreateSpell;
use crate::domain::models::Spell::Spell;

#[async_trait]
pub trait SpellRepository: Send + Sync {
    async fn CreateSpell(&self, NewSpell: &CreateSpell) -> RepositoryResult<Spell>;
    async fn GetAllSpells(&self) -> RepositoryResult<Vec<Spell>>;
    async fn GetSpell(&self, SpellID: i32) -> RepositoryResult<Spell>;
    async fn RemoveSpell(&self, SpellID: i32) -> RepositoryResult<()>;
}