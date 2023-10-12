use warp::{Filter, reply::Reply, reject::Rejection};

use crate::files::DeleteMode;

use super::{state::SharedState, rejection::HttpReject};

pub async fn uploaded((file, state): (String, SharedState)) -> Result<Box<dyn Reply>, Rejection> {

    let mut file_res = state.file_mgr.find_by_hash(file.clone())
        .map_err(|x| warp::reject::custom(HttpReject::StringError(x.to_string())))?;
    
    if file_res.is_none() {
        file_res = state.file_mgr.find_by_name(file.clone())
            .map_err(|x| warp::reject::custom(HttpReject::StringError(x.to_string())))?;
    }

    if file_res.is_none() {
        return Ok(
            Box::new(warp::reply::with_status("Not found", warp::http::StatusCode::NOT_FOUND))
        )
    }
    let file_res = file_res.unwrap();
    let data = file_res.read_unchecked().await.unwrap();
    
    match file_res.delete_mode {
        DeleteMode::Time => {
            if file_res.expired() {
                let _ = file_res.delete(state.clone()).await;
            }
        },
        DeleteMode::TimeOrDownload => {
            let _ = file_res.delete(state.clone()).await;
        }
    }

    Ok(
        Box::new(
            warp::reply::with_header(
                data,
                "Content-Type", file_res.mime
            )
        )
    )
}

pub fn get_uploaded(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("upload" / String)
        .map(move |x| (x, state.clone()))
        .and_then(uploaded)
}