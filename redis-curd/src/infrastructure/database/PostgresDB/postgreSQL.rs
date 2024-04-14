use std::thread::current;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, ManageConnection};
use diesel::r2d2;

use log::error;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<PgConnection>;
pub type DBConn = PostgresPool;


pub fn GetPGConnection(postgres_url: &str) -> DBConn {
    let Manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(postgres_url);

    let PGDBPool = match Pool::builder()
        .max_size(2)
        .build(Manager) {
        Ok(PostgresDBPool) => PostgresDBPool,
        Err(Error) => {
            error!("[{}] Error Initializing PostgreSQL Manager. Error : {}",current().name().unwrap(), Error);
            panic!("[{}] Error Initializing PostgreSQL Manager. Error : {}", current().name().unwrap(), Error);
        }
    };
    
    PGDBPool
}

