/*
 pages.rs - All the HTML pages
*/

use warp::{reply::{Reply, Html}, Filter, reject::Rejection};
use askama::Template;

#[derive(Template)]
#[template( path = "index.html" )]
struct Index {}

pub fn index() -> Html<String> {
    let rendered = Index {};
    warp::reply::html(rendered.render().unwrap())
}

pub fn get_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let index_r = warp::path::end().map(index);

    warp::any().and(index_r)
}