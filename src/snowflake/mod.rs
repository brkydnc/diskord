mod error;

use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::num::NonZeroU64;

pub use error::Error;

#[derive(Copy, Clone, Debug, Serialize)]
pub struct Snowflake {
    id: NonZeroU64,
}

impl TryFrom<u64> for Snowflake {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let id: NonZeroU64 = value.try_into().map_err(|_| Error)?;
        Ok(Self { id })
    }
}

impl From<Snowflake> for u64 {
    fn from(snowflake: Snowflake) -> u64 {
        snowflake.id.get()
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        f.write_str(&self.id.to_string())
    }
}
