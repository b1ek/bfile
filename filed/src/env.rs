/*
 env.rs - The environment loader. It loads all the .env config to a convenient `Env` struct.
 This file provides the `loadenv` function that will do just that.
 */

use std::{env::var, net::SocketAddr};

#[derive(Debug, Clone)]
pub struct Redis {
    pub pass: String,
    pub host: String,
    pub port: u16,
    pub prefix: String
}

#[derive(Debug, Clone)]
pub struct Env {
    pub logging: bool,
    pub listen: SocketAddr,
    pub redis: Redis,
}

fn get_var<T: Into<String>, O: From<String>>(name: T) -> Result<O, String> {
    let name: String = name.into();
    let v = var(name.clone());
    if v.is_err() {
        return Err(format!("Variable {name} does not exist!"));
    }
    return Ok(v.unwrap().into())
}

pub fn loadenv() -> Result<Env, Box<dyn std::error::Error>> {
    Ok(
        Env {
            logging: get_var::<&str, String>("APP_LOGGING")?.to_lowercase() == "true",
            listen: get_var::<&str, String>("APP_HOST")?.parse::<SocketAddr>().unwrap(),
            redis: Redis {
                pass: get_var("REDIS_PASS")?,
                host: get_var("REDIS_HOST")?,
                port: get_var::<&str, String>("REDIS_PORT")?.parse().unwrap(),
                prefix: get_var("REDIS_PREFIX")?
            }
        }
    )
}