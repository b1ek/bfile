
/*
 forms.rs - All the forms
*/

use std::collections::HashMap;

use warp::{Filter, reply::Reply, reject::Rejection, filters::multipart::FormData};
use futures_util::TryStreamExt;
use bytes::BufMut;

use super::state::SharedState;

pub async fn upload(form: FormData, _state: SharedState) -> Result<Box<dyn Reply>, Rejection> {

    let params: HashMap<String, String> = form.and_then(|mut field| async move {
        let mut bytes: Vec<u8> = vec![];
        while let Some(byte) = field.data().await {
            bytes.put(byte.unwrap())
        }
        
        Ok((field.name().into(), String::from_utf8_lossy(&bytes).to_string()))
    }).try_collect().await.unwrap();

    Ok(Box::new(warp::reply::json(&params)))
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post().and(
        warp::multipart::form()
            .and(warp::any().map(move || state.clone()))
            .and_then(upload)
    )
}