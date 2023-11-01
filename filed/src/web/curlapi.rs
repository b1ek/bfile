use warp::{Filter, reply::Reply, reject::Rejection};

use super::state::SharedState;

mod upload;

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    upload::get_routes(state)
}