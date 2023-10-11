
mod clean;
mod state;
mod env;

pub fn redis_conn(env: env::Env) -> Result<redis::Client, redis::RedisError> {
    log::info!("Connecting to redis DB on {}", env.redis.host);
    redis::Client::open(format!("redis://:{}@{}:{}/", env.redis.pass, env.redis.host, env.redis.port))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let env = crate::env::Env::load().unwrap();
    let statee = crate::state::State {
        redis: redis_conn(env).unwrap()
    };

    loop {
        let res = clean::clean(statee).await;
        if res.is_err() {
            log::error!("Error while cleaning")
        }
    }
}
