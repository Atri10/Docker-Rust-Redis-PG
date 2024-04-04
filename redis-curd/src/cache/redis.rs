use std::thread::current;
use redis::{Client, Commands, Connection};
use std::time::Duration;
use log::{debug, error};


pub struct RedisCache {
    connection: Connection,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Self {
        debug!("[{}] Trying to Initialize Redis at : {}", current().name().unwrap(),redis_url);

        let client: Client = match Client::open(redis_url) {
            Ok(client) => client,
            Err(err) => {
                error!("[{}] Error Encountered while Initializing Redis: {:?}",current().name().unwrap(), err);
                panic!("Error Encountered while Initializing Redis");
            }
        };

        debug!("[{}] Successfully Initialized Redis at : {}", current().name().unwrap(),redis_url);


        let connection: Connection = match client.get_connection() {
            Ok(connection) => connection,
            Err(err) => {
                error!("[{}] Error connecting to Redis Client: {:?}. Error : {}",current().name().unwrap(),client, err);
                panic!("[{}] Error connecting to Redis Client: {:?}. Error : {}", current().name().unwrap(), client, err);
            }
        };

        RedisCache { connection }
    }

    pub fn set(&mut self, key: &str, val: &str, ttl: Duration) -> bool {
        return match self.connection.set_ex(key, val, ttl.as_secs()) {
            Ok(()) => {
                debug!("[{}] Successfully set key : {} in Redis cache", current().name().unwrap(),key);
                true
            }

            Err(err) => {
                error!(target: current().name().unwrap(),"[{}] Error setting Key : {} to Redis. Error : {:?}",current().name().unwrap(),key,err);
                false
            }
        };
    }

    pub fn remove(&mut self, key: &str) -> bool {
        return match self.connection.del(key) {
            Ok(()) => {
                debug!("[{}] Successfully removed key : {} in Redis cache", current().name().unwrap(),key);
                return true;
            }
            Err(err) => {
                error!(target: current().name().unwrap(),"[{}] Error removing Key : {} to Redis. Error : {:?}",current().name().unwrap(),key,err);
                false
            }
        };
    }
}
