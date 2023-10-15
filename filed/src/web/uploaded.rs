use std::time::Duration;

use argon2::{PasswordVerifier, PasswordHash};
use base64::{alphabet, engine, Engine};
use warp::{Filter, reply::Reply, reject::Rejection};

use crate::files::DeleteMode;

use super::{state::SharedState, rejection::HttpReject};

fn btoa(base: String) -> Result<String, Box<dyn std::error::Error>> {
    let decoder = engine::GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::PAD);
    Ok(String::from_utf8(decoder.decode(base)?)?)
}

pub async fn uploaded((file, state): (String, SharedState), authorization: Option<String>) -> Result<Box<dyn Reply>, Rejection> {

    let mut file_res = state.file_mgr.find_by_hash(file.clone())
        .map_err(|x| warp::reject::custom(HttpReject::StringError(x.to_string())))?;
    
    if file_res.is_none() {
        file_res = state.file_mgr.find_by_name(file.clone())
            .map_err(|x| warp::reject::custom(HttpReject::StringError(x.to_string())))?;
    }

    if file_res.is_none() {
        return Ok(
            Box::new(warp::reply::with_status("Not found", warp::http::StatusCode::NOT_FOUND))
        )
    }
    let file_res = file_res.unwrap();

    if let Some(pass) = file_res.clone().password {
        log::debug!("File is protected by a password");
        if let Some(user_pass) = authorization {
            let user_pass = user_pass.chars().skip(6).collect::<String>();
            let user_pass = btoa(user_pass).unwrap();
            let user_pass = user_pass.split(':').collect::<Vec<&str>>();
            let user_pass = user_pass.last().unwrap().to_string();

            log::debug!("User provided a password: \"{}\"", user_pass);
            log::debug!("Halting for 5 seconds");
            tokio::time::sleep(Duration::from_secs(5)).await;

            let argon = crate::security::get_argon2();
            let hash = PasswordHash::parse(&pass, argon2::password_hash::Encoding::B64).unwrap();

            if ! argon.verify_password(user_pass.as_bytes(), &hash).is_ok() {
                log::debug!("Password doesn't match");
                return Ok(
                    Box::new(
                        warp::reply::with_status(
                            warp::reply::with_header(warp::reply::html("Invalid password"), "WWW-Authenticate", "basic"),
                            warp::http::StatusCode::UNAUTHORIZED
                        )
                    )
                )
            } else {
                log::debug!("Password match");
            }
        } else {
            log::debug!("Password not provided");
            return Ok(
                Box::new(
                    warp::reply::with_status(
                        warp::reply::with_header(warp::reply::html(""), "WWW-Authenticate", "basic realm=\"File is protected with a password. Login field is ignored\""),
                        warp::http::StatusCode::UNAUTHORIZED
                    )
                )
            )
        }
    }

    let data = file_res.read_unchecked().await.unwrap();
    
    match file_res.delete_mode {
        DeleteMode::Time => {
            if file_res.expired() {
                log::debug!("Deleting the file since it is expired");
                let _ = file_res.delete(state.clone()).await;
            }
        },
        DeleteMode::TimeOrDownload => {
            log::debug!("Deleting the file since it is a 1-download file");
            let _ = file_res.delete(state.clone()).await;
        }
    }

    Ok(
        Box::new(
            warp::reply::with_header(
                data,
                "Content-Type", file_res.mime
            )
        )
    )
}

pub fn get_uploaded(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("upload" / String)
        .map(move |x| (x, state.clone()))
        .and(warp::header::optional("Authorization"))
        .and_then(uploaded)
}