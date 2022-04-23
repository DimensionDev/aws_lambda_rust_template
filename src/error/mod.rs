use lambda_http::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Param missing: {0}")]
    ParamMissing(String),
    #[error("JSON Parse error")]
    ParseError(#[from] serde_json::error::Error),
    #[error("no body provided")]
    BodyMissing,
    #[error("Lambda HTTP error: {0}")]
    LambdaHttpError(#[from] lambda_http::http::Error),
}

impl Error {
    pub fn http_status(&self) -> StatusCode {
        match self {
            Error::ParamMissing(_) => StatusCode::BAD_REQUEST,
            Error::ParseError(_) => StatusCode::BAD_REQUEST,
            Error::BodyMissing => StatusCode::BAD_REQUEST,
            Error::LambdaHttpError(_) => StatusCode::INTERNAL_SERVER_ERROR,

        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
