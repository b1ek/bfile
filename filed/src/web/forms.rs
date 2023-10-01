
/*
 forms.rs - All the forms
*/

use std::collections::HashMap;

use askama::Template;
use warp::{Filter, reply::Reply, reject::Rejection, filters::multipart::FormData, http::StatusCode};
use futures_util::TryStreamExt;
use bytes::BufMut;
use serde::Serialize;

use crate::files::File;

use super::{state::SharedState, pages::BadActionReq, rejection::HttpReject};

#[derive(Debug, Serialize)]
struct FormElement {
    data: Vec<u8>,
    mime: String
}

pub async fn upload(form: FormData, _state: SharedState) -> Result<Box<dyn Reply>, Rejection> {

    let params: HashMap<String, FormElement> = form.and_then(|mut field| async move {
        let mut bytes: Vec<u8> = vec![];
        while let Some(byte) = field.data().await {
            bytes.put(byte.unwrap())
        }
        
        Ok((field.name().into(), FormElement { data: bytes, mime: field.content_type().unwrap_or("text/plain").to_string() }))
    }).try_collect()
      .await
      .map_err(|err| warp::reject::custom(HttpReject::WarpError(err.into())))?;

    // check that required fields exist
    let mut all_exist = true;
    let _ = vec!["delmode", "file", "filename", "password"].iter().for_each(|x| {
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

    let data = params.get("file").unwrap();
    let delmode = params.get("delmode").unwrap();
    let named = params.get("named");
    let filename = params.get("filename").unwrap();

    let file = File::create(
        data.data.clone(),
        data.mime.clone(),
        match named {
            Some(named) => {
                if String::from_utf8(named.data.clone())
                    .map_err(|err| warp::reject::custom(HttpReject::FromUtf8Error(err)))?
                    .to_string() == "on" {
                    Some(String::from_utf8(filename.data.clone()).map_err(|err| warp::reject::custom(HttpReject::FromUtf8Error(err)))?)
                } else {
                    None
                }
            },
            None => None
        },
        _state.env.clone()
    ).await;

    Ok(Box::new(warp::reply::json(&params)))
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post().and(
        warp::multipart::form()
            .and(warp::any().map(move || state.clone()))
            .and_then(upload)
    )
}