use super::request::Request;
use hyper::{
    client::{HttpConnector, ResponseFuture},
    Body, Client as HyperClient,
};
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

    fn request(&self, request: Request) -> ResponseFuture {
        self.client.request(request.to_hyper_request(&self.token))
    }
}
