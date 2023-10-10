use warp::{Filter, reply::{Reply, html}, reject::Rejection};

use super::state::SharedState;

pub async fn uploaded((file, _state): (String, SharedState)) -> Result<Box<dyn Reply>, Rejection> {
    Ok(Box::new(html(file)))
}

pub fn get_uploaded(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("upload" / String)
        .map(move |x| (x, state.clone()))
        .and_then(uploaded)
}