use serde::{Serialize, Deserialize};
use std::{error::Error, fs};

use crate::env::Env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesPolicy {
    /// Whether the uploads are enabled
    #[serde(default)]
    allow_uploads: bool,

    /// Allow custom names
    #[serde(default)]
    allow_custom_names: bool,

    /// Allow password protection
    #[serde(default)]
    allow_pass_protection: bool,

    /// Max uploads for IP (doesn't include deleted uploads)
    #[serde(default)]
    max_per_ip: usize,

    /// Default time for file to be deleted
    #[serde(default)]
    file_del_timeout: usize,

    /// Whitelisted file types
    #[serde(default)]
    type_whitelist: Option<Vec<String>>,

    /// Backlisted file types
    #[serde(default)]
    type_blacklist: Option<Vec<String>>,
}

impl Default for FilesPolicy {
    fn default() -> Self {
        FilesPolicy {
            allow_uploads: true,
            allow_custom_names: true,
            allow_pass_protection: true,
            max_per_ip: 8,
            file_del_timeout: 1800,
            type_whitelist: None,
            type_blacklist: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branding {
    /// Instance name
    #[serde(default)]
    instance_name: String,

    /// Instance motto
    #[serde(default)]
    instance_motto: String,
    
    /// Instance URL (not the bind URL). Must be Some(...)
    #[serde(default)]
    instance_url: Option<String>,
}

impl Default for Branding {
    fn default() -> Self {
        Branding {
            instance_name: "blek! File".into(),
            instance_motto: "A minute file sharing".into(),
            instance_url: None,
        }
    }
}

impl FilesPolicy {
    pub fn validate(self: &Self) -> Result<(), String> {
        Ok(())
    }
}

impl Branding {
    fn validate(self: &Self) -> Result<(), String> {
        if self.instance_url.is_none() {
            return Err("Instance url must not be empty!".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    files: FilesPolicy,
    brand: Branding
}

impl Config {

    pub fn validate(self: &Self) -> Result<(), String> {
        self.files.validate()?;
        self.brand.validate()?;

        Ok(())
    }

    pub fn load(env: Env) -> Result<Config, Box<dyn Error>> {
        let raw = fs::read_to_string(env.confpath.clone())?;
        let conf: Config = toml::from_str(&raw)?;
        conf.validate()?;
        Ok(conf)
    }
}