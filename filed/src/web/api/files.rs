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

fn is_api_pass(state: &SharedState) -> bool {
    if let Some(keys) = state.config.api.apikeys.clone() {
        keys.len() != 0
    } else {
        false
    }
}

fn check_api_pass(state: &SharedState, key: String) -> Result<(), WithStatus<Json>> {
    let mut valid = {
        if let Some(keys) = state.config.api.apikeys.clone() {
            keys.iter().find(|x| (**x) == key).is_some()
        } else {
            false
        }
    };

    if key.len() == 0 {
        valid = false
    }

    if valid {
        Ok(())
    } else {
        Err(
            warp::reply::with_status(
                json(&ErrorMessage::new(Error::APIPasswordDenied)),
                StatusCode::FORBIDDEN
            )
        )
    }
}

fn function_disabled_err() -> WithStatus<Json> {
    warp::reply::with_status(
        json(&ErrorMessage::new(Error::APIFunctionDisabled)),
        StatusCode::SERVICE_UNAVAILABLE
    )
}

pub mod get_all;
pub mod delete;
pub mod upload;