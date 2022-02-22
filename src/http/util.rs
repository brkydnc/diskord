use crate::error::Result;
use hyper::{
    body::{Body, Buf},
    Response,
};
use serde::Deserialize;

#[inline]
pub async fn deserialize_response<T>(response: Response<Body>) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    let body = response.into_body();
    let reader = hyper::body::aggregate(body).await?.reader();
    let deserialized: T = serde_json::from_reader(reader)?;
    Ok(deserialized)
}
