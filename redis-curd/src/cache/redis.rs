use std::thread::current;
use redis::{Client, Commands, Connection};
use std::time::Duration;
use log::{debug, error};

#[derive(Debug)]
pub struct RedisCache {
    client: Client,
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

        RedisCache { client }
    }

    pub fn set(&self, key: &str, val: &str, ttl: Duration) -> bool {
        let mut connection: Connection = self.get_connection();

        return match connection.set_ex(key, val, ttl.as_secs()) {
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

    fn get_connection(&self) -> Connection {
        let connection: Connection = match self.client.get_connection() {
            Ok(connection) => connection,
            Err(err) => {
                error!("[{}] Error connecting to Redis: {:?}. Error : {}",current().name().unwrap(),self, err);
                panic!("[{}] Error connecting to Redis: {:?}. Error : {}", current().name().unwrap(), self, err);
            }
        };

        connection
    }

    pub fn remove(&self, key: &str) -> bool {
        let mut connection: Connection = self.get_connection();

        return match connection.del(key) {
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
