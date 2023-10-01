/*
 pages.rs - All the HTML pages
*/

use std::{collections::HashMap, convert::Infallible};

use warp::{reply::{Reply, Html}, Filter, reject::{Rejection, Reject}};
use askama::Template;

use crate::env::Env;

use super::{state::SharedState, rejection::HttpReject};

#[derive(Template)]
#[template( path = "index.html" )]
pub struct Index {
    env: Env
}

#[derive(Template)]
#[template( path = "bad_action_req.html" )]
pub struct BadActionReq {
    env: Env
}

#[derive(Template)]
#[template( path = "uploaded.html" )]
#[allow(dead_code)]
pub struct Uploaded {
    file: String,
    env: Env
}


pub async fn uploaded(query: HashMap<String, String>, state: SharedState) -> Result<Html<String>, Rejection> {

    if ! query.contains_key("file") {
        return Err(warp::reject());
    }

    let rendered = Uploaded {
        file: query.get("file").unwrap().clone(),
        env: state.env.clone()
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
        env: state.env.clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn index_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(move || state.clone())
        .and_then(index)
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    index_f(state.clone())
        .or(uploaded_f(state.clone()))
}