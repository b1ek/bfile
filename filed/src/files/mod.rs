#![allow(unused)]

use std::{sync::Arc, error::Error, ops::Add};

use chrono::{DateTime, Local};
use num::BigUint;
use sha2::{Sha512, Digest, digest::FixedOutput};
use serde::{Serialize, Deserialize};
use tokio::fs;

use crate::env::Env;

pub mod lookup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub size: BigUint,
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
        } else {
            return Err("File already uploaded".into());
        }

        let expires = Local::now();
        expires.add(chrono::Duration::minutes(30));

        Ok(
            File {
                path: path.display().to_string(),
                size: BigUint::from(data.len()),
                name: Some(filename),
                mime,
                delete_at: expires,
                sha512: hash
            }
        )
    }
}