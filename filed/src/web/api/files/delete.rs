use std::collections::HashMap;

use warp::{reply::{Reply, json}, reject::Rejection, Filter, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::web::{state::SharedState, rejection::HttpReject, api::types::{ErrorMessage, Error}};

use super::{function_disabled_err, check_api_enabled};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteFunctionPayload {
    pub fid: String
}

pub async fn delete(state: SharedState, body: DeleteFunctionPayload) -> Result<Box<dyn Reply>, Rejection> {
    if let Err(res) = check_api_enabled(&state) {
        return Ok(Box::new(res));
    }

    if (!state.config.api.delete) || (!state.config.api.enabled) {
        return Ok(Box::new(function_disabled_err()))
    }

    let id = body.fid;
    let mut file = state.file_mgr.find_by_hash(id.clone())
        .map_err(|x| HttpReject::StringError(x.to_string()))?;

    if let None = file {
        file = state.file_mgr.find_by_name(id)
            .map_err(|x| HttpReject::StringError(x.to_string()))?;
    }

    if let None = file {
        return Ok(
            Box::new(
                warp::reply::with_status(
                    json(
                        &ErrorMessage {
                            error: Error::APIError,
                            details: Some("No file with that ID was found.".into())
                        }
                    ),
                    StatusCode::NOT_FOUND
                )
            )
        )
    }

    Ok(Box::new(json(&HashMap::<(), ()>::new())))
}

pub fn delete_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "files" / "delete")
        .map(move || state.clone())
        .and(warp::body::json())
        .and_then(delete)
}