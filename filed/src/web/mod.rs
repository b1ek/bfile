/*
 web - The part of filed that handles everything related to HTTP
 */

mod pages;

/*
 Serve the HTTP server
 */
pub async fn serve() {

    log::info!("Listening on 0.0.0.0:80");

    warp::serve(pages::get_routes()).run(([0,0,0,0], 80)).await;
}