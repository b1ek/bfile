
/*
 forms.rs - All the forms
*/

use std::collections::HashMap;

use askama::Template;
use warp::{Filter, reply::Reply, reject::Rejection, filters::multipart::FormData, http::StatusCode};
use futures_util::TryStreamExt;
use bytes::BufMut;
use serde::Serialize;

use crate::files::{File, lookup::LookupKind, DeleteMode};

use super::{state::SharedState, pages::{BadActionReq, UploadSuccessPage, self}, rejection::HttpReject};

#[derive(Debug, Serialize, Clone)]
struct FormElement {
    data: Vec<u8>,
    mime: String
}
impl FormElement {
    pub fn as_str_or_reject(self: &Self) -> Result<String, Rejection> {
        Ok(String::from_utf8(self.data.clone()).map_err(|err| warp::reject::custom(HttpReject::FromUtf8Error(err)))?)
    }
}

pub async fn upload(form: FormData, state: SharedState) -> Result<Box<dyn Reply>, Rejection> {

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
                    BadActionReq {
                        env: state.env.clone()
                    }
                        .render()
                        .map_err(|err| warp::reject::custom(HttpReject::AskamaError(err.into())))?
                ),
                StatusCode::BAD_REQUEST
            )
        ))
    }

    let check_off = FormElement { data: "off".as_bytes().to_vec(), mime: "text/plain".into() };

    let data = params.get("file").unwrap();
    let delmode = params.get("delmode").unwrap();
    let named = params.get("named");
    let filename = params.get("filename").unwrap();
    let tos_check = match params.get("tos_consent") {
        Some(v) => (*v).clone(),
        None => check_off.clone()
    };

    let protected = params.get("passworded").unwrap_or(&check_off.clone()).as_str_or_reject()?;
    let protected = protected == "on";
    let password: Option<String> = {
        let pass = params.get("password");
        if protected && pass.is_some() {
            Some(pass.unwrap().as_str_or_reject()?)
        } else {
            None
        }
    };

    let mut is_named = named.is_none();
    let tos_check = tos_check.as_str_or_reject()?;
    if tos_check != "on" {
        return Ok(
            Box::new(
                warp::reply::html(
                    pages::ErrorPage {
                        env: state.env,
                        error_text: "You must consent to the terms and conditions!".into(),
                        link: Some("/".into()),
                        link_text: Some("Go back".into())
                    }
                    .render()
                    .map_err(
                        |err|
                        warp::reject::custom(HttpReject::AskamaError(err))
                    )?
                )
            )
        )
    }

    let delmode = delmode.as_str_or_reject()?;
    if delmode != "30" && delmode != "dl" {
        return Err(warp::reject::custom(HttpReject::StringError("delmode is neither 30 or dl!".into())));
    }
    
    if named.is_some() {
        is_named = named.unwrap().as_str_or_reject()? == "on";
    }

    let file = File::create(
        data.data.clone(),
        data.mime.clone(),
        match named {
            Some(named) => {
                if named.as_str_or_reject()?
                    .to_string() == "on" {
                    Some(filename.as_str_or_reject()?)
                } else {
                    None
                }
            },
            None => None
        },
        state.env.clone(),
        {
            if delmode == "30" {
                DeleteMode::Time
            } else {
                DeleteMode::TimeOrDownload
            }
        },
        password
    ).await
     .map_err(|err| warp::reject::custom(HttpReject::StringError(err.to_string())))?;

    state.file_mgr.save(&file, {
        if is_named {
            LookupKind::ByName
        } else {
            LookupKind::ByHash
        }
    }).map_err(|err| warp::reject::custom(HttpReject::StringError(err.to_string())))?;

    let uploaded = UploadSuccessPage {
        env: state.env.clone(),
        link: file.leftmost_link()
    };

    Ok(Box::new(warp::reply::html(uploaded.render().unwrap())))

}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post().and(
        warp::multipart::form()
            .and(warp::any().map(move || state.clone()))
            .and_then(upload)
    )
}