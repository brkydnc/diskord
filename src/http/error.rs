use super::util::deserialize_response;
use crate::error::Result;
use hyper::{Body, Response, StatusCode};
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Debug, Deserialize)]
#[non_exhaustive]
struct DiscordError {
    message: String,
    code: u64,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ApiError {
    status: StatusCode,
    error: DiscordError,
}

impl ApiError {
    pub async fn try_from_response(response: Response<Body>) -> Result<Self> {
        let status = response.status();
        let error: DiscordError = deserialize_response(response).await?;

        Ok(Self { status, error })
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        f.write_str(&format!(
            "{} (code: {}, status: {})",
            &self.error.message, self.error.code, &self.status
        ))
    }
}

impl StdError for ApiError {}
