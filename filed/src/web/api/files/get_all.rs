use warp::{reply::{Reply, json}, reject::Rejection, Filter};

use crate::web::{state::SharedState, rejection::HttpReject};

use super::check_api_enabled;

pub async fn get_all(state: SharedState) -> Result<Box<dyn Reply>, Rejection> {
    if let Err(res) = check_api_enabled(&state) {
        return Ok(Box::new(res));
    }

    Ok(
        Box::new(
            json(
                &state.file_mgr.get_all(true, true)
                    .await
                    .map_err(|x| x.to_string())
                    .map_err(|x| HttpReject::StringError(x))?
            )
        )
    )
}

pub fn get_all_f(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "files" / "get_all")
        .map(move || state.clone())
        .and_then(get_all)
}