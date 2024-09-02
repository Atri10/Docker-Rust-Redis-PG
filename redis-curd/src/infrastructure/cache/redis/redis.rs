// use std::sync::Mutex;
// use std::thread::current;
// use redis::{Client, Commands, Connection};
// use std::time::Duration;
// use log::{debug, error};
// 
// 
// pub struct RedisCache {
//     connection: Mutex<Connection>,
// }
// 
// impl RedisCache {
//     pub fn new(redis_url: &str) -> Self {
//         debug!("[{:?}] Trying to Initialize Redis at : {}", current().id(),redis_url);
// 
//         let client: Client = match Client::open(redis_url) {
//             Ok(client) => client,
//             Err(err) => {
//                 error!("[{:?}] Error Encountered while Initializing Redis: {:?}",current().id(), err);
//                 panic!("Error Encountered while Initializing Redis");
//             }
//         };
// 
//         debug!("[{:?}] Successfully Initialized Redis at : {}", current().id(),redis_url);
// 
// 
//         let connection = match client.get_connection() {
//             Ok(connection) => connection,
//             Err(err) => {
//                 error!("[{:?}] Error connecting to Redis Client: {:?}. Error : {}",current().id(),client, err);
//                 panic!("[{:?}] Error connecting to Redis Client: {:?}. Error : {}", current().id(), client, err);
//             }
//         };
// 
//        
//         RedisCache { connection: connection.into() }
//     }
// 
//     pub fn set(&mut self, key: &str, val: &str, ttl: Duration) -> bool {
//         return match self.connection.lock().unwrap().set_ex(key, val, ttl.as_secs()) {
//             Ok(()) => {
//                 debug!("[{:?}] Successfully set key : {} in Redis cache", current().id(),key);
//                 true
//             }
// 
//             Err(err) => {
//                 error!(target: current().id(),"[{:?}] Error setting Key : {} to Redis. Error : {:?}",current().id(),key,err);
//                 false
//             }
//         };
//     }
// 
//     pub fn get(&self, key: &str) -> Option<String> {
//         match self.connection.lock().unwrap().get(key) {
//             Ok(value) => {
//                 if let Some(value) = value {
//                     debug!("[{:?}] Successfully retrieved value for key : {} from Redis cache", current().id(), key);
//                     Some(value)
//                 } else {
//                     debug!("[{:?}] Key : {} not found in Redis cache", current().id(), key);
//                     None
//                 }
//             }
//             Err(err) => {
//                 error!(target: current().id(),"[{:?}] Error getting value for Key : {} from Redis. Error : {:?}",current().id(),key,err);
//                 None
//             }
//         }
//     }
// 
// 
//     pub fn remove(&mut self, key: &str) -> bool {
//         return match self.connection.lock().unwrap().del(key) {
//             Ok(()) => {
//                 debug!("[{:?}] Successfully removed key : {} in Redis cache", current().id(),key);
//                 return true;
//             }
//             Err(err) => {
//                 error!(target: current().id(),"[{:?}] Error removing Key : {} from Redis. Error : {:?}",current().id(),key,err);
//                 false
//             }
//         };
//     }
// }


use std::thread::current;
use std::time::Duration;
use log::{debug, error};
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use r2d2::Pool;


#[derive(Clone)]
pub struct RedisPool {
    pool: Pool<RedisConnectionManager>,
}

impl RedisPool {
    pub fn new(RedisURI: &str) -> Self {
        let Manager: RedisConnectionManager = match RedisConnectionManager::new(RedisURI) {
            Ok(Manager) => Manager,
            Err(err) => {
                error!("[{:?}] Error Encountered while Initializing Redis Connection Manager. Error: {:?}",current().id(), err);
                panic!("Error Encountered while Initializing Redis Manager.");
            }
        };

        let pool = match Pool::builder().build(Manager) {
            Ok(pool) => pool,
            Err(err) => {
                error!("[{:?}] Error Encountered while Initializing Redis Pool. Error: {:?}", current().id(), err);
                panic!("Error Encountered while Initializing Redis Pool.");
            }
        };

        RedisPool { pool }
    }
    pub fn get_connection(&self) -> r2d2::PooledConnection<RedisConnectionManager> {
        self.pool.get().unwrap()
    }


    pub fn set(self, key: &str, val: &str, ttl: Duration) -> bool {
        let mut conn = self.get_connection();
        return match conn.set_ex(key, val, ttl.as_secs() as usize) {
            Ok(()) => {
                debug!("[{:?}] Successfully set key : {} in Redis cache", current().id(),key);
                true
            }

            Err(err) => {
                error!(target: current().name().unwrap(),"[{:?}] Error setting Key : {} to Redis. Error : {:?}",current().id(),key,err);
                false
            }
        };
    }


    pub fn get(&self, key: &str) -> Option<String> {
        match self.get_connection().get(key) {
            Ok(value) => {
                if let Some(value) = value {
                    debug!("[{:?}] Successfully retrieved value for key : {} from Redis cache", current().id(), key);
                    Some(value)
                } else {
                    debug!("[{:?}] Key : {} not found in Redis cache", current().id(), key);
                    None
                }
            }
            Err(err) => {
                error!(target: current().name().unwrap(),"[{:?}] Error getting value for Key : {} from Redis. Error : {:?}",current().id(),key,err);
                None
            }
        }
    }


    pub fn remove(&mut self, key: &str) -> bool {
        return match self.get_connection().del(key) {
            Ok(()) => {
                debug!("[{:?}] Successfully removed key : {} in Redis cache", current().id(),key);
                return true;
            }
            Err(err) => {
                error!(target: current().name().unwrap(),"[{:?}] Error removing Key : {} from Redis. Error : {:?}",current().id(),key,err);
                false
            }
        };
    }
}