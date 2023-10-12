
use redis::Commands;
use tokio::task::JoinSet;
use std::{error::Error, path::Path};

use crate::{state::State, file::File};

// check the files recorded in database
async fn check_key(key: String, mut client: redis::Client) -> bool {

    #[cfg(debug_assertions)]
    log::debug!("Checking object {}", key);

    let val: String = client.get(key.clone()).unwrap();
    let file: File = serde_json::from_str(val.as_str()).unwrap();

    if file.expired() {
        #[cfg(debug_assertions)] {
            log::debug!("Object {key} is marked for deletion because it has expired");
        }
        client.del::<String, ()>(key).unwrap();
        return true;
    }

    if ! Path::new(&file.path.clone()).exists() {
        #[cfg(debug_assertions)] {
            log::debug!("Object {key} is marked for deletion because it doesn't exist in the filesystem");
        }
        client.del::<String, ()>(key).unwrap();
        return true;
    }

    let stat = tokio::fs::metadata(file.clone().path).await.unwrap();
    if ! stat.is_file() {
        #[cfg(debug_assertions)] {
            log::debug!("Object {key} is marked for deletion because it exists in the filesystem, but is not a file");
        }
        client.del::<String, ()>(key).unwrap();
        return true;
    }

    false
}

pub async fn check_file(file: String, keys: Vec<String>, prefix: String) -> bool {
    if ! keys.iter().find(|x| x.chars().skip(prefix.len() + 4 + 2).collect::<String>() == file).is_none() {
        #[cfg(debug_assertions)] {
            log::debug!("File {file} is marked for deletion because it exists in the filesystem, but is not in the database");
        }
        let _ = tokio::fs::remove_file(file).await;
        return true
    }


    false
}

// check that all files in filesystem exist in the database
pub async fn fskeep(state: State) -> Result<(), Box<dyn Error>> {
    let mut redis = state.redis.clone();
    let keys: Vec<String> = redis.keys(format!("{}*", state.env.redis.prefix))?;
    let objects = keys.len();
    
    let mut files_s = tokio::fs::read_dir(state.env.usercont_dir).await?;
    let mut files: Vec<String> = vec![];
    while let Some(f) = files_s.next_entry().await? {
        files.push(
            f.path().into_os_string().into_string()
            .map_err(|x| format!("Couldnt parse non-utf8 encoded path: {:?}", x))?
        );
    }

    #[cfg(debug_assertions)]
    log::debug!("Got {} objects", objects);

    let mut set: JoinSet<bool> = JoinSet::new();

    for file in files {
        set.spawn(check_file(file, keys.clone(), state.env.redis.prefix.clone()));
    }

    #[cfg(debug_assertions)]
    let mut del_count = 0_u32;

    while let Some(_deleted) = set.join_next().await {
        #[cfg(debug_assertions)] {
            if _deleted.is_ok() {
                if _deleted.unwrap() {
                    del_count += 1;
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    log::debug!("Deleted {} stray files", del_count);

    Ok(())
}

pub async fn clean(state: State) -> Result<(), Box<dyn Error>> {

    let mut redis = state.redis.clone();
    let keys: Vec<String> = redis.keys(format!("{}*", state.env.redis.prefix))?;
    let objects = keys.len();
    
    #[cfg(debug_assertions)]
    log::debug!("Got {} objects", objects);
    
    let mut set: JoinSet<bool> = JoinSet::new();
    for key in keys {
        set.spawn(check_key(key, redis.clone()));
    }

    #[cfg(debug_assertions)]
    let mut del_count = 0_u32;

    while let Some(_deleted) = set.join_next().await {

        #[cfg(debug_assertions)] {
            if _deleted.is_ok() {
                if _deleted.unwrap() {
                    del_count += 1;
                }
            }
        }

    }

    #[cfg(debug_assertions)] {
        log::debug!("Deleted {} objects", del_count);
        log::debug!("Finished checking the DB, checking the filesystem...");
    }
    
    fskeep(state).await?; 

    Ok(())
}