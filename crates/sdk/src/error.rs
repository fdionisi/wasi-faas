use std::fmt;

use crate::http::HttpError;

#[derive(Debug)]
pub enum WasiFaasSdkError {
    Http(HttpError),
}

impl From<HttpError> for WasiFaasSdkError {
    fn from(err: HttpError) -> Self {
        WasiFaasSdkError::Http(err)
    }
}

impl fmt::Display for WasiFaasSdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for WasiFaasSdkError {}
