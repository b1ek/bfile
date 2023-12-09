use warp::{reply::{WithStatus, Json, json}, http::StatusCode};

use crate::web::state::SharedState;

use super::types::{ErrorMessage, Error};

fn check_api_enabled(state: &SharedState) -> Result<(), WithStatus<Json>> {
    if ! state.config.api.enabled {
        return Err(
            warp::reply::with_status(
                json(&ErrorMessage::new(Error::APIDisabled)),
                StatusCode::SERVICE_UNAVAILABLE
            )
        )
    }
    Ok(())
}

fn function_disabled_err() -> WithStatus<Json> {
    warp::reply::with_status(
        json(&ErrorMessage::new(Error::APIFunctionDisabled)),
        StatusCode::SERVICE_UNAVAILABLE
    )
}

pub mod get_all;