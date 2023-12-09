
use std::error::Error;

use redis::{Client, Commands, AsyncCommands, Connection};

use crate::env::Env;

use super::File;

#[derive(Debug, Clone)]
pub struct FileManager {
    conn: Client,
    env: Env
}

pub enum LookupKind {
    ByName,
    ByHash
}

impl FileManager {
    pub fn new(conn: Client, env: Env) -> FileManager {
        FileManager { conn, env }
    }

    async fn find_all(self: &Self, predicate: String) -> Result<Vec<File>, Box<dyn Error>> {
        let mut conn = self.conn.get_async_connection().await?;
        let found: Vec<String> = conn.keys(predicate).await?;
        let serialized: Vec<File> = 
                found.iter()
                    .map(|x| {
                        let result = serde_json::from_str(&x);
                        match result {
                            Ok(x) => Some(x),
                            Err(err) => {
                                log::error!("Error while serializing {x}: {:?}", err);
                                None
                            }
                        }
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect();
        Ok(serialized)
    }

    /// Filter options
    /// ```
    /// names: Find files with names  
    /// hash:  Find files without names  
    /// ```
    /// 
    /// If both of those are false, this function will be optimized out
    pub async fn get_all(self: &Self, names: bool, hash: bool) -> Result<Vec<File>, Box<dyn Error>> {
        if (!names) && (!hash) {
            return Ok(vec![]);
        }

        let mut conn = self.conn.get_async_connection().await?;
        let mut out = vec![];

        if names {
            self.find_all(format!("{}-name-*", self.env.redis.prefix))
                .await?
                .iter()   
                .for_each(|x| out.push(x.clone()));
        }

        if hash {
            self.find_all(format!("{}-hash-*", self.env.redis.prefix))
                .await?
                .iter()   
                .for_each(|x| out.push(x.clone()));
        }

        Ok(out)
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
        log::debug!("Saving a file with key: {key}");
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
                if let Some(name) = file.name.clone() {
                    log::debug!("Using {} as a custom file name", name);
                    return Ok(self.save_int(
                        &file,
                        format!(
                            "{}{}{}",
                            self.env.redis.prefix,
                            midfix,
                            name
                        )
                    )?)
                } else {
                    return Err("Filename can't be None when LookupKind is ByName!".into())
                }
            }
            _ => log::debug!("No custom file name detected")
        }

        self.save_int(
            &file,
            format!(
                "{}{}{}",
                self.env.redis.prefix,
                midfix,
                file.hash()
            )
        )
    }
}