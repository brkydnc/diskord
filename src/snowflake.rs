use std::fmt::{Display, Error, Formatter};
use std::num::NonZeroU64;

#[derive(Copy, Clone, Debug)]
pub struct Snowflake {
    id: NonZeroU64,
}

impl TryFrom<u64> for Snowflake {
    type Error = <NonZeroU64 as TryFrom<u64>>::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let id: NonZeroU64 = value.try_into()?;
        Ok(Self { id })
    }
}

impl From<Snowflake> for u64 {
    fn from(snowflake: Snowflake) -> u64 {
        snowflake.id.get()
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.id.to_string())
    }
}
