/*
 env.rs - The environment loader. It loads all the .env config to a convenient `Env` struct.
 This file provides the `loadenv` function that will do just that.
 */

use std::{env::var, net::SocketAddr, path::Path, fs};

pub const DEFAULT_CONFIG: &'static str = include_str!("../config/filed.toml.example");

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
    pub filedir: String,
    pub instanceurl: String,
    pub uploadspath: String,
    pub confpath: String
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
            },
            filedir: {
                let spath: String = get_var("USERCONTENT_DIR")?;
                let path = Path::new(&spath);
                if ! path.exists() {
                    fs::create_dir_all(path).map_err(|err| format!("Could not create usercontent directory: {err}"))?;
                }
                if ! path.is_dir() {
                    return Err(format!("USERCONTENT_DIR is set to \"{}\", which exists but is not a directory!", &spath).into())
                }
                spath
            },
            instanceurl: get_var("INSTANCE_URL")?,
            uploadspath: get_var("UPLOADS_PATH")?,
            confpath: {
                let spath: String = get_var("CONF_FILE").unwrap_or("/etc/filed/filed.toml".into());
                let path = Path::new(&spath);
                let mut dirpath = path.clone().components();
                dirpath.next_back();
                let dirpath = dirpath.as_path();

                if ! path.is_file() {
                    if dirpath.is_dir() {
                        log::info!("Config file does not exist, but found config directory. Trying to write the example");

                        let wrote = fs::write(path, DEFAULT_CONFIG.clone());

                        if wrote.is_err() {
                            log::info!("Could not write example because of the following error: {:?}", wrote.unwrap_err());
                            log::info!("Giving up");
                        } else {
                            log::info!("Wrote example to {}", spath);
                        }
                    }
                    return Err(format!("CONF_FILE is {}, which is not a file!", spath).into())
                }
                spath
            }
        }
    )
}

impl Env {
    pub fn usercontent_dir(self: &Self) -> Box<&Path> {
        Box::new(Path::new(&self.filedir))
    }
}