use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::num::TryFromIntError;

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        f.write_str(
            "Couldn't create Snowflake from the given integer (Snowflakes are NoneZeroU64s)",
        )
    }
}

impl StdError for Error {}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Error {
        Error
    }
}
