
/*
 forms.rs - All the forms
*/

use std::{collections::HashMap, net::IpAddr};

use askama::Template;
use warp::{Filter, reply::{Reply, with_status, html}, reject::Rejection, filters::multipart::FormData, http::StatusCode};
use futures_util::TryStreamExt;
use bytes::BufMut;
use serde::Serialize;
use warp_real_ip::real_ip;

use crate::files::{File, lookup::LookupKind, DeleteMode};

use super::{state::SharedState, pages::{UploadSuccessPage, ErrorPage}, rejection::HttpReject};

#[derive(Debug, Serialize, Clone)]
pub struct FormElement {
    pub data: Vec<u8>,
    pub mime: String
}
impl FormElement {
    
    #[allow(dead_code)]
    pub fn as_str_or_reject(self: &Self) -> Result<String, Rejection> {
        Ok(String::from_utf8(self.data.clone()).map_err(|err| warp::reject::custom(HttpReject::FromUtf8Error(err)))?)
    }

    pub fn as_atr_or_none(self: &Self) -> Option<String> {
        if let Ok(res) = String::from_utf8(self.data.clone()) {
            Some(res)
        } else {
            None
        }
    }

    pub fn is_checked(self: &Self) -> bool {
        if self.data.len() != 2 {
            return false
        }
        let data = self.data.clone();
        let on = "on".bytes().collect::<Vec<u8>>();
        data == on
    }

    pub async fn from_formdata(form: FormData) -> Result<HashMap<String, FormElement>, warp::Error> {
        form.and_then(|mut field| async move {
            let mut bytes: Vec<u8> = vec![];
            while let Some(byte) = field.data().await {
                bytes.put(byte.unwrap())
            }
            
            Ok((field.name().into(), FormElement { data: bytes, mime: field.content_type().unwrap_or("text/plain").to_string() }))
        }).try_collect()
          .await
    }
}

pub struct UploadFormData {
    pub filename: Option<String>,
    pub password: Option<String>,
    pub instancepass: Option<String>,
    pub lookup_kind: LookupKind,
    pub delmode: DeleteMode,
    pub file: Vec<u8>,
    pub mime: String,
    pub tos_consent: bool
}

impl Default for UploadFormData {
    fn default() -> Self {
        UploadFormData {
            filename: None,
            password: None,
            instancepass: None,
            lookup_kind: LookupKind::ByHash,
            delmode: DeleteMode::Time,
            file: vec![],
            mime: "application/x-octet-stream".into(),
            tos_consent: false
        }
    }
}

impl UploadFormData {

    pub fn from_formdata(data: HashMap<String, FormElement>, use_defaults: bool) -> Option<UploadFormData> {
        let mut out = Self::default();
        
        // Add a name
        match data.get("named") {
            Some(val) => {
                if val.is_checked() {
                    let name = data.get("filename")?;
                    out.filename = Some(name.as_atr_or_none()?);
                    out.lookup_kind = LookupKind::ByName
                }
            },
            None => ()
        }

        // Add a password
        match data.get("passworded") {
            Some(val) => {
                if val.is_checked() {
                    let pass = data.get("password")?;
                    out.password = Some(pass.as_atr_or_none()?);
                }
            },
            None => ()
        }

        // Delete mode
        match data.get("delmode") {
            Some(val) => {
                let val = val.data.clone();
                let is_30 = val == "30".bytes().collect::<Vec<u8>>();
                if is_30 {
                    out.delmode = DeleteMode::Time
                } else {
                    out.delmode = DeleteMode::TimeOrDownload
                }
            },
            None => {
                if ! use_defaults {
                    return None
                }
            }
        }

        match data.get("instancepass") {
            Some(val) => {
                let val = val.data.clone();
                if let Ok(pass) = String::from_utf8(val) {
                    out.instancepass = Some(pass);
                }
            },
            None => ()
        };

        let file = data.get("file")?;
        out.file = file.data.clone();
        out.mime = file.mime.clone();
        out.tos_consent = match data.get("tos_consent") {
            Some(v) => v.is_checked(),
            None => false
        };

        Some(out)
    }
}

fn bad_req_html(data: String) -> Box<dyn Reply> {
    Box::new(
        with_status(
            html(data),
            StatusCode::BAD_REQUEST
        )
    )
}

pub async fn upload(form: FormData, ip: Option<IpAddr>, state: SharedState) -> Result<Box<dyn Reply>, Rejection> {

    if ! state.config.files.allow_uploads {
        return Ok(
            Box::new(warp::redirect(warp::http::Uri::from_static("/")))
        )
    }

    let params: HashMap<String, FormElement> = FormElement::from_formdata(form).await.map_err(|x| HttpReject::WarpError(x))?;
    let formdata = UploadFormData::from_formdata(params.clone(), false);

    if let Some(formdata) = formdata {

        let mut breaks_conf = false;
        if (!state.config.files.allow_custom_names) && formdata.filename.is_some() {
            breaks_conf = true;
        }
        if (!state.config.files.allow_pass_protection) && formdata.password.is_some() {
            breaks_conf = true;
        }
        
        if breaks_conf {
            let error = ErrorPage {
                env: state.env,
                conf: state.config,
                error_text: "Attempt to set name or password when they are disabled".into(),
                link: None,
                link_text: None
            };
            return Ok(
                bad_req_html(
                    error.render()
                        .map_err(|x| HttpReject::AskamaError(x))?
                )
            );
        }
        if ! formdata.tos_consent {
            let error = ErrorPage {
                env: state.env,
                conf: state.config,
                error_text: "You must agree to the ToS".into(),
                link: None,
                link_text: None
            };

            return Ok(
                bad_req_html(
                    error.render()
                        .map_err(|x| HttpReject::AskamaError(x))?
                )
            )
        }

        if let Some(upload_pass) = state.config.files.upload_pass.clone() {

            if let Some(pass) = formdata.instancepass {
                if upload_pass != pass {
                    let error = ErrorPage {
                        env: state.env.clone(),
                        conf: state.config.clone(),
                        error_text: "Password is invalid".into(),
                        link: Some("/".into()),
                        link_text: Some("Go back".into())
                    };

                    return Ok(
                        Box::new(
                            html(
                                error.render()
                                    .map_err(|x| HttpReject::AskamaError(x))?
                            )
                        )
                    )
                }
            } else {
                let error = ErrorPage {
                    env: state.env.clone(),
                    conf: state.config.clone(),
                    error_text: "Password is not available".into(),
                    link: Some("/".into()),
                    link_text: Some("Go back".into())
                };

                return Ok(
                    Box::new(
                        html(
                            error.render()
                                .map_err(|x| HttpReject::AskamaError(x))?
                        )
                    )
                )
            }
        }

        let file = File::create(
            formdata.file,
            formdata.mime,
            formdata.filename.clone(),
            state.env.clone(),
            formdata.delmode,
            formdata.password,
            ip
        ).await
         .map_err(|x| HttpReject::StringError(x.to_string()))
         ?;
        
        state.file_mgr.save(&file, formdata.lookup_kind)
                      .map_err(|x| HttpReject::StringError(x.to_string()))?;
        
        let page = UploadSuccessPage {
            env: state.env,
            conf: state.config,
            link: match formdata.filename {
                Some(v) => v,
                None => file.hash()
            }
        };

        return Ok(
            Box::new(
                html(
                    page.render()
                        .map_err(|x| HttpReject::AskamaError(x))?
                )
            )
        )
    }

    let error = ErrorPage {
        env: state.env,
        conf: state.config,
        error_text: "Form is malformed".into(),
        link: None,
        link_text: None
    };

    return Ok(
        bad_req_html(
            error.render()
                .map_err(|x| HttpReject::AskamaError(x))?
        )
    )

}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::multipart::form())
        .and(real_ip(vec![state.env.proxy_addr]))
        .and(
            warp::any().map(move || state.clone())
        )
        .and_then(upload)
}