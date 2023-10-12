use redis::Client;

use crate::env::Env;

#[derive(Debug, Clone)]
pub struct State {
    pub redis: Client,
    pub env: Env
}