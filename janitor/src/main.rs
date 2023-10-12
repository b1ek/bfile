
mod clean;
mod state;
mod env;
mod file;

pub fn redis_conn(env: env::Env) -> Result<redis::Client, redis::RedisError> {
    log::info!("Connecting to redis DB on {}", env.redis.host);
    redis::Client::open(format!("redis://:{}@{}:{}/", env.redis.pass, env.redis.host, env.redis.port))
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)] {
        femme::with_level(log::LevelFilter::Debug);
    }
    #[cfg(not(debug_assertions))] {
        femme::with_level(log::LevelFilter::Info);
    }

    dotenvy::dotenv().unwrap();

    let env = crate::env::Env::load().unwrap();
    let statee = crate::state::State {
        redis: redis_conn(env.clone()).unwrap(),
        env: env.clone()
    };

    loop {

        #[cfg(debug_assertions)]
        log::debug!("Initiating clean process");

        let envy = env.clone();
        let res = clean::clean(statee.clone()).await;
        if res.is_err() {
            log::error!("Error while cleaning: {}", res.unwrap_err());
            log::error!("Retrying in {}", std::env::var("CLEAN_ERRDEL").unwrap());
            tokio::time::sleep(envy.clean_errdel).await;
            continue;
        }
    
        #[cfg(debug_assertions)] {
            log::debug!("Cleaned successfully");
            log::debug!("Next clean is scheduled in {}", std::env::var("CLEAN_DEL").unwrap())
        }

        tokio::time::sleep(envy.clean_errdel).await;
    }
}
