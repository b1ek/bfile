
use std::{error::Error, env::var, time::Duration};

#[derive(Debug, Clone)]
pub struct RedisEnv {
    pub pass: String,
    pub host: String,
    pub port: u16,
    pub prefix: String
}

#[derive(Debug, Clone)]
pub struct Env {
    pub redis: RedisEnv,
    pub clean_del: Duration,
    pub clean_errdel: Duration
}

impl Env {
    pub fn load() -> Result<Env, Box<dyn Error>> {
        Ok(
            Env {
                redis: RedisEnv {
                    pass: var("REDIS_PASS")?.to_string(),
                    host: var("REDIS_HOST")?.to_string(),
                    port: var("REDIS_PORT")?.parse()?,
                    prefix: var("REDIS_PASS")?.to_string()
                },
                clean_del: parse_duration::parse(var("CLEAN_DEL")?.as_str())?,
                clean_errdel: parse_duration::parse(var("CLEAN_ERRDEL")?.as_str())?
            }
        )
    }
}