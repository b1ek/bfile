/*
 pages.rs - All the HTML pages
*/

use warp::{reply::{Reply, Html}, Filter, reject::Rejection};
use askama::Template;

use super::state::SharedState;

#[derive(Template)]
#[template( path = "index.html" )]
pub struct Index {}

#[derive(Template)]
#[template( path = "bad_action_req.html" )]
pub struct BadActionReq {}


pub fn index() -> Html<String> {
    let rendered = Index {};
    warp::reply::html(rendered.render().unwrap())
}

pub fn get_routes(_state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let index_r = warp::path::end().map(index);

    warp::any().and(index_r)
}