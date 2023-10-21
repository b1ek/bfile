use serde::{Serialize, Deserialize};
use std::{error::Error, fs};

use crate::env::Env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesPolicy {
    /// Whether the uploads are enabled
    #[serde(default)]
    pub allow_uploads: bool,

    /// Allow custom names
    #[serde(default)]
    pub allow_custom_names: bool,

    /// Allow password protection
    #[serde(default)]
    pub allow_pass_protection: bool,

    /// Reson why the uploads are disabled
    /// This is shown only if allow_uploads = false
    #[serde(default)]
    pub upload_disable_reason: Option<String>,

    /// Max uploads for IP (doesn't include deleted uploads)
    #[serde(default)]
    pub max_per_ip: usize,

    /// Default time for file to be deleted
    #[serde(default)]
    pub file_del_timeout: usize,

    /// Whitelisted file types
    #[serde(default)]
    pub type_whitelist: Option<Vec<String>>,

    /// Backlisted file types
    #[serde(default)]
    pub type_blacklist: Option<Vec<String>>,
}

impl Default for FilesPolicy {
    fn default() -> Self {
        FilesPolicy {
            allow_uploads: true,
            allow_custom_names: true,
            allow_pass_protection: true,
            upload_disable_reason: None,
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
    pub instance_name: String,

    /// Instance motto
    #[serde(default)]
    pub instance_motto: String,

    /// Instance emoji, displayed like this:
    /// 🌠 blek! File
    #[serde(default)]
    pub instance_emoji: char,
}

impl Default for Branding {
    fn default() -> Self {
        Branding {
            instance_name: "blek! File".into(),
            instance_motto: "A minute file sharing".into(),
            instance_emoji: '🌠',
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
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub files: FilesPolicy,
    pub brand: Branding
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