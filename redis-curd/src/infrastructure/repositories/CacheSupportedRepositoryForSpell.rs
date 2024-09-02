use std::fmt::format;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::current;
use std::time::Duration;

use actix_threadpool::run;
use actix_web::HttpResponse;
use async_trait::async_trait;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel_migrations::{MigrationHarness};
use log::debug;

use crate::domain::constants::{MIGRATIONS, TTL};
use crate::domain::error::RepositoryError;
use crate::domain::models::Spell::{CreateSpell, Spell};
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::repositories::SpellRepository::SpellRepository;

// use crate::infrastructure::cache::redis::redis::RedisCache;
use crate::infrastructure::cache::redis::redis::RedisPool;
use crate::infrastructure::database::PostgresDB::postgreSQL::DBConn;
use crate::infrastructure::errors::CacheSupportedRepositoryError::CachedRepositoryError;
use crate::infrastructure::models::CacheSupportedModelForSpell::{CreateSpellDiesel, SpellDiesel, SpellRedis};

pub struct SpellCachedRepository {
    pub PostgresPool: Arc<DBConn>,
    pub RedisCachePool: RedisPool,
}

impl SpellCachedRepository {
    pub fn new(DBpool: Arc<DBConn>, RedisPool: RedisPool) -> Self {
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

        debug!("[DB_Insertion]: Successfully Inserted spell Data in Database. Generated Spell ID : {}", result.id);

        /*

        We are Done Inserting Data inside Postgres DB
        Now we will push this data inside Redis Cache,
        to avoid delays we can perform this in a separate thread.

         */

        let result_clone: Spell = result.clone().into();
        let NewSpellRedis: SpellRedis = SpellRedis::from(result_clone);
        let mut redisCachePool: RedisPool = self.RedisCachePool.clone();

        thread::spawn(move || {
            debug!("[Cache_Insertion]: Going to Put spell Data for Spell:{} in Cache.", NewSpellRedis.id);

            let key = format!("spell:{}", NewSpellRedis.id);
            let value = NewSpellRedis;
            let ttl = Duration::from_secs(TTL);
            redisCachePool.set(&key, serde_json::to_string(&value).unwrap().as_str(), ttl);
        });

        Ok(result.into())
    }


    async fn GetAllSpells(&self) -> RepositoryResult<Vec<Spell>> {
        use crate::infrastructure::schema::spells::dsl::{spells};

        /*
            We can not return cached response here, It is useless.
            Think WHY ?
         */

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

        let redisResult = redisCachePool.get(&key);

        match redisResult {
            None => {
                debug!("[Cache_Miss]: Spell Data Not found from the Cache, For Spell:{}",SpellID);

                use crate::infrastructure::schema::spells::dsl::{id, spells};

                let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();

                let mut result: Result<Spell, RepositoryError> = run(move || {
                    spells
                        .filter(id.eq(SpellID))
                        .first::<SpellDiesel>(&mut DBPool)
                }).await
                    .map_err(|e| CachedRepositoryError::from((e, -12)).into_inner())
                    .map(|spell| -> Spell { spell.into() });


                match result {
                    Ok(result) => {
                        debug!("[Cache_Insertion]: Going to Put spell Data for Spell:{} in Cache.", SpellID);

                        let result_clone: Spell = result.clone();
                        let NewSpellRedis: SpellRedis = SpellRedis::from(result_clone.clone());
                        let mut redisCachePool: RedisPool = self.RedisCachePool.clone();

                        thread::spawn(move || {
                            let key = format!("spell:{}", NewSpellRedis.id);
                            let value = NewSpellRedis;
                            let ttl = Duration::from_secs(TTL);
                            redisCachePool.set(&key, serde_json::to_string(&value).unwrap().as_str(), ttl);
                        });

                        Ok(result_clone)
                    }

                    Err(error) => {
                        Err(error)
                    }
                }
            }
            Some(RedisSpell) => {
                debug!("[Cache_Hit]: Spell Data for Spell:{} Found from the Cache.", SpellID);

                let result: SpellRedis = serde_json::from_str::<SpellRedis>(&RedisSpell).unwrap();
                let DeserializedSpell: Spell = SpellRedis::into(result);
                Ok(DeserializedSpell)
            }
        }
    }

    async fn RemoveSpell(&self, SpellID: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::spells::dsl::{id, spells};

        let mut DBPool: PooledConnection<ConnectionManager<PgConnection>> = self.PostgresPool.get().unwrap();
        let mut redisCachePool = self.RedisCachePool.clone();


        debug!("[DB_Deletion]: Going to Remove spell Data for Spell:{} from Database.", SpellID);

        let _result = run(move || diesel::delete(spells)
            .filter(id.eq(SpellID))
            .execute(&mut DBPool))
            .await
            .map_err(|e| CachedRepositoryError::from((e, -13)).into_inner());

        thread::spawn(move || {
            debug!("[Cache_Deletion]: Going to Remove spell Data for Spell:{} from Cache.", SpellID);
            let key = format!("spell:{}", SpellID);
            redisCachePool.remove(&key);
        });


        Ok(())
    }
}