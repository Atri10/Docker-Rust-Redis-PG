use std::env;
use std::sync::Arc;
use crate::domain::constants::POSTGRESQL_DB_URI;
use crate::domain::services::SpellService::SpellService;
use crate::infrastructure::database::PostgresDB::postgreSQL::GetPGConnection;
use crate::infrastructure::repositories::SpellRepository::SpellDieselRepository;
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
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}