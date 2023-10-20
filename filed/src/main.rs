#![forbid(unsafe_code)]
#![warn(clippy::suspicious)]
#![warn(clippy::correctness)]

mod files;
mod env;
mod web;
mod db;
mod config;

pub mod security;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let envy = env::loadenv().map_err(|err| format!("Could not load env: {err}")).unwrap();
    let conf = config::types::Config::load(envy.clone()).unwrap();

    // set up logging
    if envy.logging {
        #[cfg(debug_assertions)] {
            femme::with_level(femme::LevelFilter::Debug)
        }
        #[cfg(not(debug_assertions))] {
            femme::with_level(femme::LevelFilter::Info)
        }
    } else {
        femme::with_level(femme::LevelFilter::Off);
    }

    web::serve(envy, conf).await;
}
