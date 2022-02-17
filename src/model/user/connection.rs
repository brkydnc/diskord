use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

// TODO: Implement `integrations` field, after implementing server integrations
// TODO: Use enum to represent `visibility` field

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Connection {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
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
