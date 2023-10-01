
use redis::Client;

use crate::{env::Env, files::lookup::FileManager};

#[derive(Debug, Clone)]
pub struct SharedState {
    pub redis_cli: Client,
    pub env: Env,
    pub file_mgr: FileManager
}
