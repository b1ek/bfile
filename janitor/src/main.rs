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
#[allow(dead_code)]
enum CleanResult {
    Skip,
    Break,
    ScheduleNext(chrono::DateTime<chrono::Local>)
}

async fn clean(env: env::Env, state: State) -> CleanResult {
    #[cfg(debug_assertions)]
    log::debug!("Initiating clean process");

    let res = clean::clean(state.clone()).await;
    if res.is_err() {
        log::error!("Error while cleaning: {}", res.unwrap_err());
        return CleanResult::ScheduleNext(chrono::Local::now() + env.clean_errdel);
    }

    #[cfg(debug_assertions)] {
        log::debug!("Cleaned successfully");
    }
    
    CleanResult::ScheduleNext(chrono::Local::now() + env.clean_del)
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
            CleanResult::ScheduleNext(next) => {
                #[cfg(debug_assertions)] {
                    log::debug!("Next run is scheduled at {} ({})", next.format("%d-%m-%Y %H:%M:%S"), next.timestamp());
                }
                tokio::time::sleep((next - chrono::Local::now()).to_std().unwrap()).await;
            },
            _ => {}
        }
    }

    log::info!("Main loop broke");
}
