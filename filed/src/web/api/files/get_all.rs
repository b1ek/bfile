use warp::{reply::{Reply, json}, reject::Rejection, Filter, http::StatusCode};

use crate::web::{state::SharedState, rejection::HttpReject};

use super::super::types::{ErrorMessage, Error};

pub async fn get_all(state: SharedState) -> Result<Box<dyn Reply>, Rejection> {
    if ! state.config.api.enabled {
        return Ok(
            Box::new(
                warp::reply::with_status(
                    json(&ErrorMessage::new(Error::APIDisabled)),
                    StatusCode::SERVICE_UNAVAILABLE
                )
            )
        )
    }

    Ok(
        Box::new(
            json(
                &state.file_mgr.get_all(true, true)
                    .await
                    .map_err(|x| x.to_string())
                    .map_err(|x| HttpReject::StringError(x))?
            )
        )
    )
}

pub fn get_all_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "files" / "get_all")
        .map(move || state.clone())
        .and_then(get_all)
}