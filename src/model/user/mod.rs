use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

// TODO: Use enums to represent some of the fields (e.g flags, premium_type)

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct User {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub username: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub discriminator: u16,
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u64>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u64>,
    pub premium_type: Option<u8>,
    pub public_flags: Option<u64>,
}
