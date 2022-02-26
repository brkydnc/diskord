use crate::http::ApiError;
use crate::snowflake::Error as SnowflakeError;
use hyper::Error as HyperError;
use serde_json::Error as SerdeJsonError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FormatResult};
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

type Cause = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<InnerError>,
}

struct InnerError {
    kind: Kind,
    cause: Option<Cause>,
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Kind {
    Hyper,
    Api,
    SerdeJson,
    Snowflake,
}

impl Error {
    pub(super) fn new(kind: Kind) -> Self {
        Self {
            inner: InnerError { kind, cause: None }.into(),
        }
    }

    pub(super) fn with(mut self, cause: impl Into<Cause>) -> Self {
        self.inner.cause = Some(cause.into());
        self
    }

    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::Hyper => "hyper error",
            Kind::Api => "discord api error",
            Kind::SerdeJson => "serde_json error",
            Kind::Snowflake => "snowflake error",
        }
    }

    pub fn kind(&self) -> Kind {
        self.inner.kind
    }

    pub fn is_hyper(&self) -> bool {
        matches!(self.inner.kind, Kind::Hyper)
    }

    pub fn is_api(&self) -> bool {
        matches!(self.inner.kind, Kind::Api)
    }

    pub fn is_serde_json(&self) -> bool {
        matches!(self.inner.kind, Kind::SerdeJson)
    }

    pub fn is_snowflake(&self) -> bool {
        matches!(self.inner.kind, Kind::Snowflake)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let mut f = f.debug_tuple("diskord::Error");
        f.field(&self.inner.kind);

        if let Some(ref cause) = self.inner.cause {
            f.field(cause);
        }

        f.finish()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        if let Some(ref cause) = self.inner.cause {
            write!(f, "{}: {}", self.description(), cause)
        } else {
            f.write_str(self.description())
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .cause
            .as_ref()
            .map(|cause| &**cause as &(dyn StdError + 'static))
    }
}

impl From<HyperError> for Error {
    fn from(cause: HyperError) -> Error {
        Error::new(Kind::Hyper).with(cause)
    }
}

impl From<ApiError> for Error {
    fn from(cause: ApiError) -> Error {
        Error::new(Kind::Api).with(cause)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(cause: SerdeJsonError) -> Error {
        Error::new(Kind::SerdeJson).with(cause)
    }
}

impl From<SnowflakeError> for Error {
    fn from(cause: SnowflakeError) -> Error {
        Error::new(Kind::Snowflake).with(cause)
    }
}
