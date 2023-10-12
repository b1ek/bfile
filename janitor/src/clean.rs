
use redis::Commands;
use tokio::task::JoinSet;
use std::{error::Error, path::Path};

use crate::{state::State, file::File};

async fn check_key(key: String, mut client: redis::Client) -> bool {

    #[cfg(debug_assertions)]
    log::debug!("Checking object {}", key);

    let val: String = client.get(key.clone()).unwrap();
    let file: File = serde_json::from_str(val.as_str()).unwrap();

    if ! Path::new(&file.path.clone()).exists() {
        #[cfg(debug_assertions)] {
            log::debug!("Object {key} is marked for deletion because it doesn't exist in the filesystem");
        }
        client.del::<String, ()>(key).unwrap();
        return true;
    }

    let stat = tokio::fs::metadata(file.clone().path).await.unwrap();
    if ! stat.is_file() {
        client.del::<String, ()>(key).unwrap();
        return true;
    }

    false
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
    let mut del_count: u32 = 0;

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
    log::debug!("Deleted {} objects", del_count);

    Ok(())
}