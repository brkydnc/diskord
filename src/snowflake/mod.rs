mod error;

use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::num::NonZeroU64;
use std::str::FromStr;

pub use error::Error;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Snowflake(NonZeroU64);

struct SnowflakeVisitor;

impl<'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut Formatter) -> FormatResult {
        write!(formatter, "a snowflake (non-zero u64) as a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let id = NonZeroU64::from_str(s)
            .map_err(|_| de::Error::invalid_value(Unexpected::Str(s), &self))?;

        Ok(Snowflake(id))
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(SnowflakeVisitor)
    }
}

impl TryFrom<u64> for Snowflake {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let id: NonZeroU64 = value.try_into().map_err(|_| Error)?;
        Ok(Self(id))
    }
}

impl From<Snowflake> for u64 {
    fn from(snowflake: Snowflake) -> u64 {
        snowflake.0.get()
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        f.write_str(&self.0.to_string())
    }
}
