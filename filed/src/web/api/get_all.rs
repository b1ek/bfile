use warp::{reply::Reply, reject::Rejection, Filter};

use crate::web::state::SharedState;

pub async fn get_all(_state: SharedState) -> Result<Box<dyn Reply>, Rejection> {
    Ok(Box::new(warp::reply::json(&String::from("aaaaa"))))
}

pub fn get_all_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "get_all")
        .map(move || state.clone())
        .and_then(get_all)
}