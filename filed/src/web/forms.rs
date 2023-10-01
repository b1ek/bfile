
/*
 forms.rs - All the forms
*/

use std::collections::HashMap;

use askama::Template;
use warp::{Filter, reply::{Reply, Html}, reject::Rejection, filters::multipart::FormData, http::StatusCode, Error};
use futures_util::TryStreamExt;
use bytes::BufMut;

use super::{state::SharedState, pages::BadActionReq, rejection::HttpReject};

pub async fn upload(form: FormData, _state: SharedState) -> Result<Box<dyn Reply>, Rejection> {

    let params: HashMap<String, String> = form.and_then(|mut field| async move {
        let mut bytes: Vec<u8> = vec![];
        while let Some(byte) = field.data().await {
            bytes.put(byte.unwrap())
        }
        
        Ok((field.name().into(), String::from_utf8_lossy(&bytes).to_string()))
    }).try_collect()
      .await
      .map_err(|err| warp::reject::custom(HttpReject::WarpError(err.into())))?;

    // check that required fields exist
    let mut all_exist = true;
    let _ = vec!["delmode", "file"].iter().for_each(|x| {
        let field = x.to_string();
        if ! params.contains_key(&field) {
            all_exist = false;
        }
    });

    if ! all_exist {
        return Ok(Box::new(
            warp::reply::with_status(
                warp::reply::html(
                    BadActionReq {}
                        .render()
                        .map_err(|err| warp::reject::custom(HttpReject::AskamaError(err.into())))?
                ),
                StatusCode::BAD_REQUEST
            )
        ))
    }

    Ok(Box::new(warp::reply::json(&params)))
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post().and(
        warp::multipart::form()
            .and(warp::any().map(move || state.clone()))
            .and_then(upload)
    )
}