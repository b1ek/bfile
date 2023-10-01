/*
 pages.rs - All the HTML pages
*/

use std::{collections::HashMap, convert::Infallible};

use warp::{reply::{Reply, Html}, Filter, reject::{Rejection, Reject}};
use askama::Template;

use super::{state::SharedState, rejection::HttpReject};

#[derive(Template)]
#[template( path = "index.html" )]
pub struct Index {}

#[derive(Template)]
#[template( path = "bad_action_req.html" )]
pub struct BadActionReq {}

#[derive(Template)]
#[template( path = "uploaded.html" )]
#[allow(dead_code)]
pub struct Uploaded {
    file: String
}


pub async fn uploaded(query: HashMap<String, String>) -> Result<Html<String>, Rejection> {

    if ! query.contains_key("file") {
        return Err(warp::reject());
    }

    let rendered = Uploaded {
        file: query.get("file").unwrap().clone()
    };
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn uploaded_f() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path("uploaded")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(uploaded)
}

pub async fn index() -> Result<Html<String>, Rejection> {
    let rendered = Index {};
    Ok(warp::reply::html(rendered.render().map_err(|err| warp::reject::custom(HttpReject::AskamaError(err)))?))
}

pub fn index_f() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path::end().and_then(index)
}

pub fn get_routes(_state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    index_f()
        .or(uploaded_f())
}