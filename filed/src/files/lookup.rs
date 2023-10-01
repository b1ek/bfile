
use std::error::Error;

use redis::{Client, Commands};

use crate::env::Env;

use super::File;

pub struct FileFinder {
    conn: Client,
    env: Env
}

pub enum LookupKind {
    ByName,
    ByHash
}

impl FileFinder {
    pub fn new(conn: Client, env: Env) -> FileFinder {
        FileFinder { conn, env }
    }
    fn find(self: &Self, key: String) -> Result<Option<File>, Box<dyn Error>> {
        let mut conn = self.conn.get_connection()?;
        
        if ! conn.exists(&key)? {
            return Ok(None)
        }

        let data: String = conn.get(&key)?;
        Ok(Some(serde_json::from_str(data.as_str())?))
    }
    pub fn find_by_name(self: &Self, name: String) -> Result<Option<File>, Box<dyn Error>> {
        Ok(self.find(format!("{}-name-{}", self.env.redis.prefix, name))?)
    }
    pub fn find_by_hash(self: &Self, hash: String) -> Result<Option<File>, Box<dyn Error>> {
        Ok(self.find(format!("{}-hash-{}", self.env.redis.prefix, hash))?)
    }

    fn save_int(self: &Self, file: &File, key: String) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.get_connection()?;
        conn.set(key, serde_json::to_string(&file)?)?;
        Ok(())
    }

    pub fn save(self: &Self, file: &File, kind: LookupKind) -> Result<(), Box<dyn Error>> {
        let file = file.clone();
        let midfix = match kind {
            LookupKind::ByName => "-name-",
            LookupKind::ByHash => "-hash-"
        };
        
        match kind {
            LookupKind::ByName => {
                if (&file).name.is_none() {
                    return Err("Filename can't be None when LookupKind is ByName!".into())
                }
            }
            _ => ()
        }

        self.save_int(
            &file,
            format!(
                "{}{}{}",
                self.env.redis.prefix,
                midfix,
                match kind {
                    LookupKind::ByName => (&file).name.as_ref().unwrap().clone(),
                    LookupKind::ByHash => (&file).hash()
                }
            )
        )
    }
}