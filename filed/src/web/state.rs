
use redis::Client;

use crate::{env::Env, files::lookup::FileManager, config::types::Config};

#[derive(Debug, Clone)]
pub struct SharedState {
    pub redis_cli: Client,
    pub file_mgr: FileManager,

    pub env: Env,
    pub config: Config
}
