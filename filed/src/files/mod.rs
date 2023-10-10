#![allow(unused)]

use std::{sync::Arc, error::Error, ops::Add};

use chrono::{DateTime, Local};
use sha2::{Sha512, Digest, digest::FixedOutput};
use serde::{Serialize, Deserialize};
use tokio::fs;

use crate::env::Env;

pub mod lookup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub size: usize,
    pub name: Option<String>,
    pub mime: String,
    pub delete_at: DateTime<Local>,
    sha512: String
}

impl File {
    pub fn comp_hash(self: &Self, other: &Sha512) -> bool {
        let mut hash = other.clone();
        hex::encode(hash.finalize_fixed()) == self.sha512
    }
    pub fn hash(self: &Self) -> String {
        self.sha512.clone()
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

    pub async fn create(data: Vec<u8>, mime: String, name: Option<String>, env: Env) -> Result<File, Box<dyn Error>> {

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
                sha512: hash
            }
        )
    }
}