use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    APIDisabled,
    APIFunctionDisabled,
    APIError,
    APIPasswordDenied
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error: Error,
    pub details: Option<String>,
}

impl ErrorMessage {
    pub fn new(error: Error) -> ErrorMessage {
        ErrorMessage {
            details: match error {
                Error::APIDisabled          => Some("API is disabled by the administrator. Please contact them for further details".into()),
                Error::APIFunctionDisabled  => Some("This API function is disabled by the administrator. Please contact them for further details.".into()),
                Error::APIError             => Some("An error has occured while executing the API request".into()),
                Error::APIPasswordDenied    => Some("API password authorization has been denied.".into())
            },
            error,
        }
    }
}