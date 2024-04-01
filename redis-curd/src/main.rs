use std::thread::current;
use std::time::Duration;
use dotenv::dotenv;
use crate::cache::redis::RedisCache;
use log::{debug, info, LevelFilter, warn};


mod cache;

fn main() {
    dotenv().ok();

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    env_logger::Builder::new().
        filter_level(LevelFilter::Trace).
        format_timestamp(Some(env_logger::TimestampPrecision::Nanos)).
        init();

    debug!("[{}] Going to Start CURD App", current().name().unwrap());


    let redis_cache_client: RedisCache = RedisCache::new(&*redis_url);

    let key = "TestKey";
    let value = "TestVal";
    let ttl = Duration::from_secs(3600);

    redis_cache_client.set(key, value, ttl);
    redis_cache_client.remove(key);
}