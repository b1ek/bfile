/*
 web - The part of filed that handles everything related to HTTP
 */

use std::env::current_dir;

use static_dir::static_dir;
use warp::{Filter, reply::Reply, reject::Rejection};

use crate::{env::Env, files::lookup::FileManager};

mod pages;
mod forms;
pub mod state;
mod rejection;
mod api;
mod uploaded;

use state::SharedState;

pub fn routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let staticpath = current_dir().unwrap();
    let staticpath = staticpath.to_str().unwrap().to_string() + "/static";

    pages::get_routes(state.clone())
        .or(forms::get_routes(state.clone()))
        .or(api::get_routes(state.clone()))
        .or(uploaded::get_uploaded(state))
        .or(static_dir!("static"))
        .or(warp::fs::dir(staticpath.to_string()))
}

/*
 Serve the HTTP server
 */
pub async fn serve(env: Env) {

    log::info!("Listening on {}", env.listen.to_string());

    let redis_cli = crate::db::redis_conn(env.clone()).unwrap();
    let state = SharedState {
        redis_cli: redis_cli.clone(),
        env: env.clone(),
        file_mgr: FileManager::new(redis_cli, env.clone())
    };

    warp::serve(routes(state)).run(env.listen).await;
}