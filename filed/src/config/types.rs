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

    /// Upload password
    #[serde(default)]
    pub upload_pass: Option<String>,

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
            upload_pass: None,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APISettings {
    /// If API is enabled
    #[serde(default)]
    pub enabled: bool,

    /// Password
    #[serde(default)]
    pub apikeys: Option<Vec<String>>,

    /// Whether /api/get_all is enabled
    #[serde(default)]
    pub get_all: bool,

    /// Whether to return only the
    /// user IP's files on /api/get_all
    #[serde(default)]
    pub get_all_own_only: bool,

    /// Whether /api/delete is enabled
    #[serde(default)]
    pub delete: bool,

    /// If /api/delete can delete any file,
    /// not only the own file
    /// (with API key provided)
    #[serde(default)]
    pub sudo_delete: bool,

    /// Whether /api/upload is enabled
    #[serde(default)]
    pub upload: bool,

    /// Whether curlapi is enabled
    #[serde(default)]
    pub curlapi: bool
}

impl Default for APISettings {
    fn default() -> Self {
        APISettings {
            enabled: true,
            apikeys: None,
            get_all: true,
            get_all_own_only: true,
            delete: false,
            sudo_delete: false,
            upload: false,
            curlapi: true
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

impl APISettings {
    fn validate(self: &Self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub files: FilesPolicy,
    pub brand: Branding,
    pub api: APISettings
}

impl Config {

    pub fn validate(self: &Self) -> Result<(), String> {
        self.files.validate()?;
        self.brand.validate()?;
        self.api  .validate()?;

        Ok(())
    }

    pub fn load(env: Env) -> Result<Config, Box<dyn Error>> {
        let raw = fs::read_to_string(env.confpath.clone())?;
        let conf: Config = toml::from_str(&raw)?;
        conf.validate()?;
        Ok(conf)
    }
}