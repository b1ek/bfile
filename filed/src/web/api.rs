use warp::{reply::Reply, reject::Rejection, Filter};
use serde::{Serialize, Deserialize};

use self::files::{get_all::get_all_f, delete::delete_f, upload::upload_f};

use super::state::SharedState;

mod files;
mod types;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIError {
    error: String
}

pub fn api_root() -> Box<dyn Reply> {
    let err = APIError {
        error: "You have called the API root of a blek! File instance. Refer to https://git.blek.codes/blek/bfile.git for documentation.".into()
    };
    Box::new(warp::reply::json(&err))
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api")
        .and(warp::path::end())
        .map(api_root)
        .or(get_all_f(state.clone()))
        .or(delete_f(state.clone()))
        .or(upload_f(state))
}