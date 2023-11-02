use warp::{Filter, reply::Reply, reject::Rejection};

use super::state::SharedState;

mod upload;
mod help;

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    upload::get_routes(state.clone())
        .or(help::get_routes(state))
}