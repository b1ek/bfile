use state::State;


mod clean;
mod state;
mod env;
mod file;

pub fn redis_conn(env: env::Env) -> Result<redis::Client, redis::RedisError> {
    log::info!("Connecting to redis DB on {}", env.redis.host);
    redis::Client::open(format!("redis://:{}@{}:{}/", env.redis.pass, env.redis.host, env.redis.port))
}

#[derive(Debug, Clone)]
enum CleanResult {
    Skip,
    #[allow(dead_code)]
    Break,
    Ok
}

async fn clean(env: env::Env, state: State) -> CleanResult {
    #[cfg(debug_assertions)]
    log::debug!("Initiating clean process");

    let envy = env.clone();
    let res = clean::clean(state.clone()).await;
    if res.is_err() {
        log::error!("Error while cleaning: {}", res.unwrap_err());
        log::error!("Retrying in {}", std::env::var("CLEAN_ERRDEL").unwrap());
        log::debug!("Next clean will run at {}", chrono::Local::now() + env.clean_errdel);
        tokio::time::sleep(envy.clean_errdel).await;
        return CleanResult::Skip;
    }

    #[cfg(debug_assertions)] {
        log::debug!("Cleaned successfully");
        log::debug!("Next clean is scheduled in {}", std::env::var("CLEAN_DEL").unwrap());
        log::debug!("Next clean will run at {}", chrono::Local::now() + env.clean_del);
    }

    tokio::time::sleep(envy.clean_errdel).await;

    CleanResult::Ok
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
    let state = crate::state::State {
        redis: redis_conn(env.clone()).unwrap(),
        env: env.clone()
    };

    log::info!("Initalizing initial clean");
    let cl = clean(env.clone(), state.clone()).await;
    log::info!("Initial clean exited with status {:?}", cl);

    loop {
        let res = clean(env.clone(), state.clone()).await;
        
        match res {
            CleanResult::Break => {
                break
            },
            _ => {}
        }
    }

    log::info!("Main loop broke");
}
