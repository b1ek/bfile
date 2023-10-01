
use redis::Client;

#[derive(Debug, Clone)]
pub struct SharedState {
    pub redis_cli: Client
}
