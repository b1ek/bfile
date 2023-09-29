/*
 pages.rs - All the HTML pages
*/

use warp::{reply::{Reply, Html}, Filter, reject::Rejection};

pub fn index() -> Html<String> {
    warp::reply::html("<b>hiiii ^^</b>".into())
}

pub fn get_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {

    let index_r = warp::path::end().map(index);

    warp::any().and(index_r)
}