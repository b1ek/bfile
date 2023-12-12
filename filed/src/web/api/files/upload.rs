use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use warp::{reply::{Reply, with_status, json}, http::StatusCode, reject::Rejection, Filter, filters::multipart::FormData};

use crate::web::{state::SharedState, forms::FormElement, rejection::HttpReject, api::types::{ErrorMessage, Error}};

use super::{is_api_pass, check_api_pass};

#[derive(Serialize, Deserialize)]
struct UploadAPIMetadata {
    sha512: String,
    name: Option<String>,
    pass: Option<String>
}

struct UploadAPIPayload {
    file: Vec<u8>,
    instance_pass: Option<String>,
    metadata: UploadAPIMetadata
}

impl Default for UploadAPIPayload {
    fn default() -> Self {
        Self {
            file: vec![],
            instance_pass: None,
            metadata: UploadAPIMetadata {
                sha512: "".into(),
                name: None,
                pass: None
            }
        }
    }
}

impl UploadAPIPayload {
    pub fn from_form(data: HashMap<String, FormElement>) -> Option<UploadAPIPayload> {

        let mut out = Self::default();

        let file = data.get("file");
        let instance_pass = data.get("instance_pass");
        let metadata = data.get("metadata");

        let mut fields_set = false;

        // required fields
        if let Some(file) = file {
            if let Some(metadata) = metadata {
                if let Some(metadata) = metadata.as_atr_or_none() {
                    out.file = file.data.clone();
                    if let Ok(metadata) = serde_json::from_str(&metadata) {
                        out.metadata = metadata;
                    }
                    fields_set = true;
                }
            }
        }

        // optional ones
        if let Some(pass) = instance_pass {
            if let Some(pass) = pass.as_atr_or_none() {
                out.instance_pass = Some(pass);
            }
        }

        if ! fields_set {
            None
        } else {
            Some(out)
        }
    }
}

pub async fn upload(state: SharedState, data: FormData) -> Result<Box<dyn Reply>, Rejection> {

    let data = FormElement::from_formdata(data)
        .await;

    if let Err(err) = data {
        return Ok(
            Box::new(
                with_status(
                    json(
                        &ErrorMessage {
                            error: Error::APIError,
                            details: Some(
                                format!("Error while parsing payload: {err}")
                            )
                        }
                    ),
                    StatusCode::BAD_REQUEST
                )
            )
        )
    }

    let data = data.unwrap(); // this is guaranteed to be `Ok` at this point

    let payload = UploadAPIPayload::from_form(data);
    if let Some(payload) = payload {

        if is_api_pass(&state) {
            if let Err(res) = check_api_pass(
                &state,
                match payload.instance_pass {
                    Some(x) => x,
                    None => "".into()
                }
            ) {
                return Ok(Box::new(res))
            }
        }

    }

    Ok(
        Box::new(
            with_status(
                json(&ErrorMessage {
                    error: Error::APIError,
                    details: Some("Request payload invalid".into())
                }),
                StatusCode::BAD_REQUEST
            )
        )
    )
}

pub fn upload_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "files" / "upload")
        .and(warp::post())
        .map(move || state.clone())
        .and(warp::multipart::form())
        .and_then(upload)
}
