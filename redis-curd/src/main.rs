use actix_web::{HttpServer};
use dotenv::dotenv;
use log::LevelFilter;
use redis_curd::create_app::create_app;


mod tests;
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    env_logger::Builder::new().
        filter_level(LevelFilter::Debug).
        format_timestamp(Some(env_logger::TimestampPrecision::Nanos)).
        init();
    
    
    let server = HttpServer::new(move || { create_app() }).bind(("0.0.0.0", 8080))?;
    server.run().await
}