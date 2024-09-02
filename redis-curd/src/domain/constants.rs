use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub const POSTGRESQL_DB_URI: &str = "DATABASE_URL";
pub const LOG_CONFIG_PATH: &str = "LOG_CONFIG_PATH";
pub const REDIS_URI: &str = "REDIS_URL";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const TTL: u64 = 3600;