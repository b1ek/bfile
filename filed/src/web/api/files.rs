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

pub mod get_all;