use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub const POSTGRESQL_DB_URI: &str = "DATABASE_URL";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");