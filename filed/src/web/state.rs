
use redis::Client;

use crate::env::Env;

#[derive(Debug, Clone)]
pub struct SharedState {
    pub redis_cli: Client,
    pub env: Env
}
