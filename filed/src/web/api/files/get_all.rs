use std::net::IpAddr;

use warp::{reply::{Reply, json, with_status}, reject::Rejection, Filter, http::StatusCode};
use warp_real_ip::real_ip;

use crate::web::{state::SharedState, api::types::{ErrorMessage, Error}};

use super::{check_api_enabled, function_disabled_err};

pub async fn get_all(state: SharedState, ip: Option<IpAddr>) -> Result<Box<dyn Reply>, Rejection> {
    if let Err(res) = check_api_enabled(&state) {
        return Ok(Box::new(res))
    }

    if (!state.config.api.get_all) || (!state.config.api.enabled) {
        return Ok(Box::new(function_disabled_err()))
    }

    let found = 
        state.file_mgr.get_all(true, true)
            .await
            .map_err(|x| x.to_string());
    
    if let Err(err) = found {
        return Ok(
            Box::new(
                with_status(
                    json(
                        &ErrorMessage {
                            error: Error::APIError,
                            details: Some(
                                format!("Error while getting all files: {err}")
                            )
                        }
                    ),
                    StatusCode::INTERNAL_SERVER_ERROR
                )
            )
        );
    } else {

        let mut found = found.unwrap();

        if state.config.api.get_all_own_only {
            found = found
                .iter()
                .filter(
                    |x| {
                        if let Some(owner) = x.uploader_ip {
                            if let Some(caller) = ip {
                                println!("{owner} {caller}");
                                return owner == caller
                            }
                        }
                        false
                    }
                ).map(|x| x.clone()).collect();
        }

        Ok(
            Box::new(
                json(
                    &found
                )
            )
        )
    }
}

pub fn get_all_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {

    let proxy = state.env.proxy_addr;

    warp::path!("api" / "files" / "get_all")
        .map(move || state.clone())
        .and(real_ip(vec![proxy]))
        .and_then(get_all)
}