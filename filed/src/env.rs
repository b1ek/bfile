/*
 env.rs - The environment loader. It loads all the .env config to a convenient `Env` struct.
 This file provides the `loadenv` function that will do just that.
 */

use std::{env::var, net::{SocketAddr, ToSocketAddrs, IpAddr}, path::Path, fs};

pub const DEFAULT_CONFIG: &'static str = include_str!("../config/filed.toml.example");

#[derive(Debug, Clone)]
pub struct Redis {
    pub pass: String,
    pub host: String,
    pub port: u16,
    pub prefix: String
}

#[derive(Debug, Clone)]
pub struct VersionData {
    pub commit: String
}
impl Default for VersionData {
    fn default() -> Self {
        VersionData {
            commit: env!("COMMIT_HASH").to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    pub logging: bool,
    pub listen: SocketAddr,
    pub proxy_addr: IpAddr,
    pub redis: Redis,
    pub filedir: String,
    pub instanceurl: String,
    pub uploadspath: String,
    pub confpath: String,
    pub version: VersionData
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
            proxy_addr: {
                let env_var = get_var::<&str, String>("PROXY_IP")?;

                let ip = env_var.parse::<IpAddr>();
                let ret =
                if let Ok(ip) = ip {
                    if ip == IpAddr::from([127, 0, 0, 1]) {
                        log::warn!("Proxy address is 127.0.0.1. No proxy will be trusted")
                    }
                    if ip == IpAddr::from([0, 0, 0, 0]) {
                        log::warn!("Proxy address is 0.0.0.0. All proxies will be trusted.");
                        #[cfg(not(debug_assertions))]
                            log::warn!("The warning above will not work well with production mode! Please consider setting the proxy address to a proper IP.")
                    }
                    ip
                } else {
                    let mut env_var = env_var;
                    
                    // add port if not added
                    if env_var.split(":").collect::<Vec<&str>>().len() == 1 {
                        env_var.push_str(":80");
                    }

                    let sock = env_var.to_socket_addrs();
                    if let Err(err) = sock {
                        return Err(format!("Can't resolve {env_var}: {:?}", err).into());
                    }
                    let mut addrs = sock.unwrap();
                    if addrs.len() == 0 {
                        return Err(format!("{env_var} resolved to nothing").into());
                    }
                    let addr = addrs.next().unwrap().ip();
                    addr
                };
                #[cfg(debug_assertions)] {
                    if ret != IpAddr::from([ 127, 0, 0, 1 ]) {
                        log::debug!("Proxy ip is {}", ret)
                    }
                }
                ret
            },
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
                let mut dirpath = path.components();
                dirpath.next_back();
                let dirpath = dirpath.as_path();

                if ! path.is_file() {

                    log::error!("Config file is not a file");
                    log::info!("Trying to recover from error");

                    if ! dirpath.is_dir() {
                        log::info!("The config file directory does not exist. Trying to create it");

                        let created = fs::create_dir_all(dirpath);
                        if created.is_err() {
                            log::warn!("Could not create the config directory: {:?}", created.unwrap_err());
                        } else {
                            log::info!("Created the config directory");
                        }
                    }

                    if dirpath.is_dir() {
                        log::info!("Config file does not exist, trying to write the example");

                        let wrote = fs::write(path, DEFAULT_CONFIG);

                        if wrote.is_err() {
                            log::warn!("Could not write example because of the following error: {:?}", wrote.unwrap_err());
                        } else {
                            log::info!("Wrote example to {}", spath);
                        }
                    }

                    log::info!("Giving up");
                    return Err(format!("CONF_FILE is {}, which is not a file!", spath).into())
                }
                spath
            },
            version: VersionData::default()
        }
    )
}

impl Env {
    pub fn usercontent_dir(self: &Self) -> Box<&Path> {
        Box::new(Path::new(&self.filedir))
    }
}