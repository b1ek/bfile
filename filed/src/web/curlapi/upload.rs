use std::net::IpAddr;

use warp::{filters::multipart::FormData, reply::{Reply, with_status}, reject::Rejection, Filter};
use warp_real_ip::real_ip;

use crate::{web::{state::SharedState, forms::{FormElement, UploadFormData}, rejection::HttpReject}, files::File};

pub async fn upload(form: FormData, ip: Option<IpAddr>, state: SharedState) -> Result<Box<dyn Reply>, Rejection> {
    if ! state.config.files.allow_uploads {
        return Ok(
            Box::new(
                with_status(
                    match state.config.files.upload_disable_reason {
                        Some(reason) => format!("Uploads are disabled for the following reason:\n{reason}"),
                        None => "Uploads are disabled.".into()
                    },
                    warp::http::StatusCode::SERVICE_UNAVAILABLE
                )
            )
        )
    }

    let params = FormElement::from_formdata(form)
        .await
        .map_err(|x| HttpReject::WarpError(x))?;

    let formdata = UploadFormData::from_formdata(params, true);
    if let Some(formdata) = formdata {

        let mut breaks_conf = false;
        if (!state.config.files.allow_custom_names) && formdata.filename.is_some() {
            breaks_conf = true;
        }
        if (!state.config.files.allow_pass_protection) && formdata.password.is_some() {
            breaks_conf = true;
        }

        if breaks_conf {
            return Ok(
                Box::new(
                    with_status(
                        "Attempt to set name or password when they are disabled".to_string(),
                        warp::http::StatusCode::BAD_REQUEST
                    )
                )
            );
        }

        if let Some(pass) = state.config.files.upload_pass {
            let mut pass_valid = false;
            if let Some(upass) = formdata.instancepass {
                pass_valid = upass == pass;
            } else {
                pass_valid = false
            }

            if ! pass_valid {
                return Ok(
                    Box::new(
                        with_status(
                            "Invalid instance password".to_string(),
                            warp::http::StatusCode::BAD_REQUEST
                        )
                    )
                );
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
        ).await.map_err(|x| HttpReject::StringError(x.to_string()))?;

        state.file_mgr.save(&file, formdata.lookup_kind).map_err(|x| HttpReject::StringError(x.to_string()))?;

        return Ok(
            Box::new(
                format!(
                    concat!(
                        "File uploaded successfully.\n",
                        "It is available via this link:\n\n",

                        "{}/upload/{}"
                    ),
                    state.env.instanceurl,
                    urlencoding::encode(
                        match formdata.filename {
                            Some(name) => name,
                            None => file.hash()
                        }.as_str()
                    )
                )
            )
        );

    } else {
        Ok(
            Box::new(
                with_status(
                    "Invalid form".to_string(),
                    warp::http::StatusCode::BAD_REQUEST
                )
            )
        )
    }
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::any()
        .and(warp::path!("curlapi" / "upload"))
        .and(warp::multipart::form())
        .and(real_ip(vec![state.env.proxy_addr]))
        .and(
            warp::any()
                .map(move || state.clone())
        )
        .and_then(upload)
}