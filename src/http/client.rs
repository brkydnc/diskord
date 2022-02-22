use crate::http::route::Route;
use hyper::{
    client::{HttpConnector, ResponseFuture},
    Body, Client as HyperClient, Request,
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

    fn request(&self, route: Route) -> ResponseFuture {
        let (method, path) = route.method_and_path();
        let request = Request::builder()
            .method(method)
            .uri(format!("https://discord.com/api/v9{}", path))
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", r#"DiscordBot ("", "0.1.0")"#)
            .body(Body::empty())
            .expect("Error building request");

        self.client.request(request)
    }
}
