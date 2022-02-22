use super::{error::ApiError, request::Request};
use crate::error::Result;
use hyper::{body::Body, client::HttpConnector, Client as HyperClient, Response as HyperResponse};
use hyper_tls::HttpsConnector;

pub struct Client {
    client: HyperClient<HttpsConnector<HttpConnector>>,
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        let https = HttpsConnector::new();
        let client = HyperClient::builder().build::<_, Body>(https);

        Self {
            client,
            token: token.to_string(),
        }
    }

    async fn request(&self, request: Request) -> Result<HyperResponse<Body>> {
        let response = self
            .client
            .request(request.to_hyper_request(&self.token))
            .await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(ApiError::try_from_response(response).await?.into())
        }
    }
}
