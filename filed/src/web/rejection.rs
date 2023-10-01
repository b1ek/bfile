
#[derive(Debug)]
pub enum HttpReject {
    WarpError(warp::Error),
    AskamaError(askama::Error)
}
impl warp::reject::Reject for HttpReject {}