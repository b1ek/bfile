use std::{collections::HashMap, net::IpAddr};

use warp::{reply::{Reply, json, with_status}, reject::Rejection, Filter, http::StatusCode};
use serde::{Serialize, Deserialize};
use warp_real_ip::real_ip;

use crate::{web::{state::SharedState, rejection::HttpReject, api::types::{ErrorMessage, Error}}, files::File};

use super::{function_disabled_err, check_api_enabled};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteFunctionPayload {
    pub fid: String,
    pub api_key: Option<String>
}

pub async fn delete(state: SharedState, body: DeleteFunctionPayload, ip: Option<IpAddr>) -> Result<Box<dyn Reply>, Rejection> {
    if let Err(res) = check_api_enabled(&state) {
        return Ok(Box::new(res));
    }

    if (!state.config.api.delete) || (!state.config.api.enabled) {
        return Ok(Box::new(function_disabled_err(StatusCode::UNAUTHORIZED)))
    }

    let mut sudo_authorized = false;
    let mut blocked = false;

    if let Some(keys) = state.config.api.apikeys.clone() {
        if let Some(key) = body.api_key {
            
            if keys.contains(&key) {
                sudo_authorized = true;
                blocked = false;
            } else {
                sudo_authorized = false;
                blocked = true;
            }

        } else {
            sudo_authorized = false;
            blocked = true
        }
    }

    if ! sudo_authorized {
        if ip.is_none() { // need the ip if sudo is not authorized
            blocked = true // to check if the file is the own file
        }
    }

    let ip = ip.unwrap();

    let id = body.fid;
    let mut file = state.file_mgr.find_by_hash(id.clone())
        .map_err(|x| HttpReject::StringError(x.to_string()))?;

    if let None = file {
        file = state.file_mgr.find_by_name(id)
            .map_err(|x| HttpReject::StringError(x.to_string()))?;
    }

    if let None = file.clone() {
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
    
    let file: File = file.unwrap();

    if let Some(uploader) = file.uploader_ip {
        if uploader != ip && (!sudo_authorized) {
            blocked = true;
        }
    } else {
        blocked = true;
    }

    if blocked {
        return Ok(
            Box::new(
                with_status(
                    json(
                        &ErrorMessage {
                            error: Error::APIPasswordDenied,
                            details: Some(
                                "Request has been denied for one of the following reasons: password auth did not pass, file was uploaded by someone else, the instance does not allow deleting files via the API".into()
                            )
                        }
                    ),
                    StatusCode::UNAUTHORIZED
                )
            )
        )
    }

    file.delete(state).await;

    Ok(Box::new(json(&HashMap::<(), ()>::new())))
}

pub fn delete_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {

    let proxy_ip = state.env.proxy_addr;

    warp::path!("api" / "files" / "delete")
        .map(move || state.clone())
        .and(warp::body::json())
        .and(real_ip(vec![proxy_ip]))
        .and_then(delete)
}