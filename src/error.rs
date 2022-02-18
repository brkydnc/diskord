use hyper::Error as HyperError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FormatResult};

pub type Result<T> = std::result::Result<T, Error>;

type Cause = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<ErrorImpl>,
}

struct ErrorImpl {
    kind: Kind,
    cause: Option<Cause>,
}

#[derive(Debug)]
#[non_exhaustive]
pub(super) enum Kind {
    Hyper,
}

impl Error {
    pub(super) fn new(kind: Kind) -> Self {
        Self {
            inner: Box::new(ErrorImpl { kind, cause: None }),
        }
    }

    pub(super) fn with(mut self, cause: impl Into<Cause>) -> Self {
        self.inner.cause = Some(cause.into());
        self
    }

    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::Hyper => "hyper error",
        }
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
