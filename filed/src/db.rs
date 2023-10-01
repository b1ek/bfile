/*
 db.rs - The database adapter
*/

use redis::{Client, RedisError};

use crate::env::Env;

pub fn redis_conn(env: Env) -> Result<Client, RedisError> {
    log::info!("Connecting to redis DB on {}", env.redis.host);
    redis::Client::open(format!("redis://:{}@{}:{}/", env.redis.pass,    env.redis.host, env.redis.port))
}