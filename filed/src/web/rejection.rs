use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum HttpReject {
    WarpError(warp::Error),
    AskamaError(askama::Error),
    FromUtf8Error(FromUtf8Error),
    StringError(String)
}
impl warp::reject::Reject for HttpReject {}