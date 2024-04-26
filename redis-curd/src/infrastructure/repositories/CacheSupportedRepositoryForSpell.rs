use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::current;
use std::time::Duration;

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

use crate::infrastructure::cache::redis::redis::RedisCache;
use crate::infrastructure::database::PostgresDB::postgreSQL::DBConn;
use crate::infrastructure::errors::CacheSupportedRepositoryError::CachedRepositoryError;
use crate::infrastructure::models::CacheSupportedModelForSpell::{CreateSpellDiesel, SpellDiesel, SpellRedis};

pub struct SpellCachedRepository {
    pub PostgresPool: Arc<DBConn>,
    pub RedisCachePool: Arc<Mutex<RedisCache>>,
}

impl SpellCachedRepository {
    pub fn new(DBpool: Arc<DBConn>, RedisPool: Arc<Mutex<RedisCache>>) -> Self {
        SpellCachedRepository { PostgresPool: DBpool, RedisCachePool: RedisPool }
    }

    pub fn run_migrations(pool: Arc<DBConn>) {
        let mut DbPool: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
        DbPool.run_pending_migrations(MIGRATIONS).expect("Error running diesel migrations");

        debug!("[{}] Migrations Executed Successfully", current().name().unwrap());
    }
}

#[async_trait]
impl SpellRepository for SpellCachedRepository {
    async fn CreateSpell(&self, NewSpell: &CreateSpell) -> RepositoryResult<Spell> {
        use crate::infrastructure::schema::spells::dsl::{spells};
        let NewSpellDiesel: CreateSpellDiesel = CreateSpellDiesel::from(NewSpell.clone());
        let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();
        let mut result: SpellDiesel = run(move || diesel::insert_into(spells)
            .values(NewSpellDiesel)
            .get_result(&mut DBPool))
            .await
            .map_err(|e| CachedRepositoryError::from((e, -10)).into_inner())?;

        /*

        We are Done Inserting Data inside Postgres DB
        Now we will push this data inside Redis Cache,
        to avoid delays we can perform this in a separate thread.

         */

        let result_clone: Spell = result.clone().into();
        let NewSpellRedis: SpellRedis = SpellRedis::from(result_clone);
        let redisCachePool: Arc<Mutex<RedisCache>> = self.RedisCachePool.clone();

        thread::spawn(move || {
            let key = format!("spell:{}", NewSpellRedis.id);
            let value = NewSpellRedis;
            let ttl = Duration::from_secs(3600);
            redisCachePool.lock().unwrap().set(&key, serde_json::to_string(&value).unwrap().as_str(), ttl);
        });

        Ok(result.into())
    }


    async fn GetAllSpells(&self) -> RepositoryResult<Vec<Spell>> {
        use crate::infrastructure::schema::spells::dsl::{spells};

        let pool = self.PostgresPool.clone();
        let result: Result<Vec<SpellDiesel>, RepositoryError> = run(move || {
            let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
            spells.load::<SpellDiesel>(&mut DBPool)
        }).await
            .map_err(|e| CachedRepositoryError::from((e, -11)).into_inner());

        result.map(|spells_diesel| spells_diesel.into_iter().map(|spell_diesel| spell_diesel.into()).collect())
    }

    async fn GetSpell(&self, SpellID: i32) -> RepositoryResult<Spell> {
        let redisCachePool = self.RedisCachePool.clone();
        let key = format!("spell:{}", SpellID);

        let redisResult = redisCachePool.lock().unwrap().get(&key);

        match redisResult {
            None => {
                debug!("Spell Data Not found from the Cache");

                use crate::infrastructure::schema::spells::dsl::{id, spells};

                let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();
                
                let mut result: Result<Spell, RepositoryError> = run(move || {
                    spells
                        .filter(id.eq(SpellID))
                        .first::<SpellDiesel>(&mut DBPool)
                }).await
                    .map_err(|e| CachedRepositoryError::from((e, -12)).into_inner())
                    .map(|spell| -> Spell { spell.into() });


                debug!("Cache Miss: Going to Push Spell in the Cache");

                let result_clone: Spell = result.unwrap().clone();
                let NewSpellRedis: SpellRedis = SpellRedis::from(result_clone.clone());
                let redisCachePool: Arc<Mutex<RedisCache>> = self.RedisCachePool.clone();

                thread::spawn(move || {
                    let key = format!("spell:{}", NewSpellRedis.id);
                    let value = NewSpellRedis;
                    let ttl = Duration::from_secs(3600);
                    redisCachePool.lock().unwrap().set(&key, serde_json::to_string(&value).unwrap().as_str(), ttl);
                });

                Ok(result_clone)
            }
            Some(RedisSpell) => {
                debug!("Spell Data Found from the Cache");

                let result: SpellRedis = serde_json::from_str::<SpellRedis>(&RedisSpell).unwrap();
                let DeserializedSpell: Spell = SpellRedis::into(result);
                Ok(DeserializedSpell)
            }
        }
    }

    async fn RemoveSpell(&self, SpellID: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::spells::dsl::{id, spells};
        let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();


        let result = run(move || diesel::delete(spells)
            .filter(id.eq(SpellID))
            .execute(&mut DBPool))
            .await
            .map_err(|e| CachedRepositoryError::from((e, -13)).into_inner());

        Ok(())
    }
}