#![allow(unused)]

use std::{sync::Arc, error::Error, ops::Add};

use argon2::{PasswordHash, password_hash::SaltString, Params, PasswordHasher};
use chrono::{DateTime, Local};
use redis::AsyncCommands;
use sha2::{Sha512, Digest, digest::FixedOutput};
use serde::{Serialize, Deserialize};
use tokio::{fs, join};

use crate::{env::Env, web::state::SharedState};

pub mod lookup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub size: usize,
    pub name: Option<String>,
    pub mime: String,
    pub delete_at: DateTime<Local>,
    pub delete_mode: DeleteMode,
    pub password: Option<String>, // argon2id hash
    sha512: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteMode {
    Time,
    TimeOrDownload
}

impl File {
    pub fn expired(self: &Self) -> bool {
        self.delete_at > chrono::Local::now()
    }

    pub fn get_redis_key(self: &Self, prefix: String) -> String {
        format!(
            "{}{}{}",
            prefix,
            match self.name {
                Some(_) => "-name-",
                None => "-hash-"
            },
            self.clone().name.unwrap_or(self.hash())
        )
    }

    pub async fn delete(self: &Self, state: SharedState) -> Result<(), Box<dyn Error>> {
        let mut redis = state.redis_cli.get_tokio_connection().await?;    
        let (r1, r2) = join!(
            tokio::fs::remove_file(self.path.clone()),
            redis.del::<String, String>(self.get_redis_key(state.env.redis.prefix))
        );
        
        r1?; r2?;
        
        Ok(())
    }

    pub fn comp_hash(self: &Self, other: &Sha512) -> bool {
        let mut hash = other.clone();
        hex::encode(hash.finalize_fixed()) == self.sha512
    }
    pub fn hash(self: &Self) -> String {
        self.sha512.clone()
    }
    pub fn leftmost_link(self: &Self) -> String {
        if self.name.is_none() {
            self.hash()
        } else {
            self.name.clone().unwrap()
        }
    }

    pub async fn read(self: &Self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.read_unchecked().await?;
        let hash = hex::encode(Sha512::digest(data.as_slice()));
        if self.hash() != hash {
            return Err("File is corrupted".into());
        }
        Ok(data)
    }

    pub async fn read_unchecked(self: &Self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = fs::read(self.path.clone()).await?;
        if self.size == data.len() {
            return Ok(data);
        }
        
        let mut ndata = Vec::<u8>::new();
        for byte in data.iter() {
            ndata.push(byte.clone());
            if ndata.len() == self.size {
                break;
            }
        }
        
        Ok(ndata)
    }

    pub async fn create(data: Vec<u8>, mime: String, name: Option<String>, env: Env, delete_mode: DeleteMode, password: Option<String>) -> Result<File, Box<dyn Error>> {

        let mut filename = String::new();
        let mut hash = Sha512::new();
        hash.update(&data);
        let hash = hex::encode(hash.finalize_fixed());

        match name {
            Some(name) =>   filename = name,
            None =>                 filename = hash.clone()
        }

        let path = env.usercontent_dir().join(&filename);
        if ! path.exists() {
            fs::write(&path, &data).await;
        }

        let expires = Local::now();
        expires.add(chrono::Duration::minutes(30));

        Ok(
            File {
                path: path.display().to_string(),
                size: data.len(),
                name: Some(filename),
                mime,
                delete_at: expires,
                delete_mode,
                sha512: hash,
                password: match password {
                    Some(pass) => {
                        // todo!("Remove possible panics on this one");
                        let argon = crate::security::get_argon2();
                        let salt = SaltString::generate(&mut rand::thread_rng());
                        let hash = argon.hash_password(pass.bytes().collect::<Vec<u8>>().as_slice(), &salt).unwrap();
                        
                        Some(hash.serialize().to_string())
                    },
                    None => None
                }
            }
        )
    }
}