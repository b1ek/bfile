#![forbid(unsafe_code)]
#![warn(clippy::suspicious)]
#![warn(clippy::correctness)]

mod files;
mod env;
mod web;
mod db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let envy = env::loadenv().unwrap();

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

    web::serve(envy).await;
}
