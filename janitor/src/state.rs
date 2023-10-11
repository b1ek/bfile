use redis::Client;

pub struct State {
    pub redis: Client
}