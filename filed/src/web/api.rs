use warp::{reply::Reply, reject::Rejection, Filter};

use super::state::SharedState;

mod get_all;

pub fn api_root() -> Box<dyn Reply> {
    Box::new(warp::reply::json(&String::from("{ error: \"You have called the API root of a blek! File instance. Refer to https://git.blek.codes/blek/bfile.git for documentation.\" }")))
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let api = warp::path!("api");
    let api = api
        .and(warp::path::end())
        .map(api_root)
        .or(get_all::get_all_f(state));
    api
}