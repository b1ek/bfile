/*
 web - The part of filed that handles everything related to HTTP
 */

use crate::env::Env;

mod pages;

/*
 Serve the HTTP server
 */
pub async fn serve(env: Env) {

    log::info!("Listening on {}", env.listen.to_string());

    warp::serve(pages::get_routes()).run(env.listen).await;
}