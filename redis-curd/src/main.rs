use actix_web::{HttpServer};
use dotenv::dotenv;
use log::LevelFilter;
use redis_curd::create_app::create_app;
use std::{env, thread};
use redis_curd::domain::constants::LOG_CONFIG_PATH;

mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let LogConfigFile = env::var(LOG_CONFIG_PATH).expect(&*format!("{value} must be set", value = LOG_CONFIG_PATH));

    log4rs::init_file(LogConfigFile, Default::default()).unwrap();

    // // 
    // env_logger::Builder::new().
    //     filter_level(LevelFilter::Debug).
    //     format_timestamp(Some(env_logger::TimestampPrecision::Nanos)).
    //     try_init();
   

    let server = HttpServer::new(move || { create_app() }).bind(("0.0.0.0", 8080))?;


    server.run().await
}