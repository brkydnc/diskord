use crate::snowflake::Snowflake;
use serde::Deserialize;

// TODO: Implement `integrations` field, after implementing server integrations
// TODO: Use enum to represent `visibility` field

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Connection {
    pub id: Snowflake,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub revoked: Option<bool>,
    #[serde(skip)]
    pub integrations: Vec<()>,
    pub verified: bool,
    pub friend_sync: bool,
    pub show_activity: bool,
    pub visibility: u8,
}
