use std::env;
use std::sync::Arc;
use crate::domain::constants::{POSTGRESQL_DB_URI, REDIS_URI};
use crate::domain::services::SpellService::SpellService;
use crate::infrastructure::cache::redis::redis::RedisCache;
use crate::infrastructure::database::PostgresDB::postgreSQL::GetPGConnection;
use crate::infrastructure::repositories::CacheSupportedRepositoryForSpell::SpellCachedRepository;
use crate::infrastructure::repositories::DieselRepositoryForSpell::SpellDieselRepository;
use crate::services::SpellService::SpellServiceImpl;

pub struct Container {
    pub SpellService: Arc<dyn SpellService>,
}

impl Container {
    pub fn new() -> Self {
        let DB_URI = env::var(POSTGRESQL_DB_URI).expect(&*format!("{value} must be set", value = POSTGRESQL_DB_URI));
        
        let SpellRepository = Arc::new(SpellDieselRepository::new(Arc::new(GetPGConnection(DB_URI.as_str()))));
        SpellDieselRepository::run_migrations(SpellRepository.PostgresPool.clone());
        
        let SpellService = Arc::new(SpellServiceImpl::new(SpellRepository));
        
        Container { SpellService }
    }

    pub fn CacheSupportedInfra() -> Self {
        let DB_URI = env::var(POSTGRESQL_DB_URI).expect(&*format!("{value} must be set", value = POSTGRESQL_DB_URI));
        let RED_URI = env::var(REDIS_URI).expect(&*format!("{value} must be set", value = REDIS_URI));
        
        let SpellRepository = Arc::new(SpellCachedRepository::new(
            Arc::new(GetPGConnection(DB_URI.as_str())),
            Arc::new(RedisCache::new(&RED_URI).into()),
        ));
        
        SpellCachedRepository::run_migrations(SpellRepository.PostgresPool.clone());
        
        let SpellService = Arc::new(SpellServiceImpl::new(SpellRepository));

        Container { SpellService }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}