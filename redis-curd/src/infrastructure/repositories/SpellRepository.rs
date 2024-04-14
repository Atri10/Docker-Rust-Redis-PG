use std::sync::Arc;
use std::thread::current;

use actix_threadpool::run;
use async_trait::async_trait;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel_migrations::{MigrationHarness};
use log::debug;

use crate::domain::constants::MIGRATIONS;
use crate::domain::error::RepositoryError;
use crate::domain::models::Spell::{CreateSpell, Spell};
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::repositories::SpellRepository::SpellRepository;

use crate::infrastructure::database::PostgresDB::postgreSQL::DBConn;
use crate::infrastructure::errors::DieselRepositoryError;
use crate::infrastructure::models::Spell::{CreateSpellDiesel, SpellDiesel};

pub struct SpellDieselRepository {
    pub PostgresPool: Arc<DBConn>,
}

impl SpellDieselRepository {
    pub fn new(pool: Arc<DBConn>) -> Self {
        SpellDieselRepository { PostgresPool: pool }
    }
    
    pub fn run_migrations(pool: Arc<DBConn>) {
        let mut DbPool: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
        DbPool.run_pending_migrations(MIGRATIONS).expect("Error running diesel migrations");

        debug!("[{}] Migrations Executed Successfully", current().name().unwrap());
    }
}

#[async_trait]
impl SpellRepository for SpellDieselRepository {
    async fn CreateSpell(&self, NewSpell: &CreateSpell) -> RepositoryResult<Spell> {
        use crate::infrastructure::schema::spells::dsl::{spells};
        let NewSpellDiesel: CreateSpellDiesel = CreateSpellDiesel::from(NewSpell.clone());
        let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();
        let result: SpellDiesel = run(move || diesel::insert_into(spells)
            .values(NewSpellDiesel)
            .get_result(&mut DBPool))
            .await
            .map_err(|e| DieselRepositoryError::from((e, -10)).into_inner())?;

        Ok(result.into())
    }

    async fn GetAllSpells(&self) -> RepositoryResult<Vec<Spell>> {
        use crate::infrastructure::schema::spells::dsl::{spells};

        let pool = self.PostgresPool.clone();
        let result: Result<Vec<SpellDiesel>, RepositoryError> = run(move || {
            let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
            spells.load::<SpellDiesel>(&mut DBPool)
        }).await
            .map_err(|e| DieselRepositoryError::from((e, -11)).into_inner());

        result.map(|spells_diesel| spells_diesel.into_iter().map(|spell_diesel| spell_diesel.into()).collect())
    }

    async fn GetSpell(&self, SpellID: i32) -> RepositoryResult<Spell> {
        use crate::infrastructure::schema::spells::dsl::{id, spells};
        let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();

        let result = run(move || spells.filter(id.eq(SpellID))
            .first::<SpellDiesel>(&mut DBPool)
        ).await
            .map_err(|e| DieselRepositoryError::from((e, -12)).into_inner())
            .map(|spell| -> Spell { spell.into() });

        result
    }

    async fn RemoveSpell(&self, SpellID: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::spells::dsl::{id, spells};
        let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();


        let result = run(move || diesel::delete(spells)
            .filter(id.eq(SpellID))
            .execute(&mut DBPool))
            .await
            .map_err(|e| DieselRepositoryError::from((e, -13)).into_inner());

        Ok(())
    }
}