/*
 web - The part of filed that handles everything related to HTTP
 */

use warp::Filter;

mod pages;

/*
 Serve the HTTP server
 */
pub async fn serve() {

    log::info!("Listening on 0.0.0.0:80");

    // let hello = warp::any().map(|| "Hi");

    warp::serve(pages::get_routes()).run(([0,0,0,0], 80)).await;
}