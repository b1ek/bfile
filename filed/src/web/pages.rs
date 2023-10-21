/*
 pages.rs - All the HTML pages
*/

use std::collections::HashMap;

use warp::{reply::{Reply, Html}, Filter, reject::Rejection};
use askama::Template;

use crate::{env::Env, config::types::Config};

use super::{state::SharedState, rejection::HttpReject};

#[derive(Template)]
#[template( path = "index.html" )]
pub struct Index {
    pub env: Env,
    pub conf: Config
}

#[derive(Template)]
#[template( path = "bad_action_req.html" )]
pub struct BadActionReq {
    pub env: Env,
    pub conf: Config
}

#[derive(Template)]
#[template( path = "uploaded.html" )]
#[allow(dead_code)]
pub struct Uploaded {
    file: String,
    pub env: Env,
    pub conf: Config
}

#[derive(Template)]
#[template( path = "passworded-files.html" )]
#[allow(dead_code)]
pub struct PasswordedFilesHelpPage {
    pub env: Env,
    pub conf: Config
}

#[derive(Template)]
#[template( path = "upload_success.html" )]
#[allow(dead_code)]
pub struct UploadSuccessPage {
    pub env: Env,
    pub conf: Config,
    pub link: String
}


#[derive(Template)]
#[template( path = "authors.html" )]
#[allow(dead_code)]
pub struct AuthorsPage {
    pub env: Env,
    pub conf: Config
}

#[derive(Template)]
#[template( path = "license.html" )]
#[allow(dead_code)]
pub struct LicensePage {
    pub env: Env,
    pub conf: Config
}

#[derive(Template)]
#[template( path = "tos.html" )]
#[allow(dead_code)]
pub struct TOSPage {
    pub env: Env,
    pub conf: Config
}


#[derive(Template)]
#[template( path = "error.html" )]
#[allow(dead_code)]
pub struct ErrorPage {
    pub env: Env,
    pub conf: Config,
    pub error_text: String,
    pub link: Option<String>,
    pub link_text: Option<String>
}

pub async fn uploaded(query: HashMap<String, String>, state: SharedState) -> Result<Html<String>, Rejection> {

    if ! query.contains_key("file") {
        return Err(warp::reject());
    }

    let rendered = Uploaded {
        file: query.get("file").unwrap().clone(),
        env: state.env.clone(),
        conf: state.config.clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn uploaded_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path("uploaded")
        .and(warp::query::<HashMap<String, String>>())
        .and(
            warp::any().map(move || state.clone())
        )
        .and_then(uploaded)
}

pub async fn index(state: SharedState) -> Result<Html<String>, Rejection> {
    let rendered = Index {
        env: state.env.clone(),
        conf: state.config.clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn index_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(move || state.clone())
        .and_then(index)
}

pub async fn passworded(state: SharedState) -> Result<Html<String>, Rejection> {
    let rendered = PasswordedFilesHelpPage {
        env: state.env.clone(),
        conf: state.config.clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn passworded_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("passworded-files")
        .and(warp::path::end())
        .map(move || state.clone())
        .and_then(passworded)
}

pub async fn authors(state: SharedState) -> Result<Html<String>, Rejection> {
    let rendered = AuthorsPage {
        env: state.env.clone(),
        conf: state.config.clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn authors_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("authors")
        .and(warp::path::end())
        .map(move || state.clone())
        .and_then(authors)
}

pub async fn license(state: SharedState) -> Result<Html<String>, Rejection> {
    let rendered = LicensePage {
        env: state.env.clone(),
        conf: state.config.clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn license_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("license")
        .and(warp::path::end())
        .map(move || state.clone())
        .and_then(license)
}

pub async fn tos(state: SharedState) -> Result<Html<String>, Rejection> {
    let rendered = TOSPage {
        env: state.env.clone(),
        conf: state.config.clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn tos_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tos")
        .and(warp::path::end())
        .map(move || state.clone())
        .and_then(tos)
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    index_f(state.clone())
        .or(uploaded_f(state.clone()))
        .or(passworded_f(state.clone()))
        .or(authors_f(state.clone()))
        .or(license_f(state.clone()))
        .or(tos_f(state))
}