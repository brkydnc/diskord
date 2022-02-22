use super::route::Route;
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Body, Request as HyperRequest,
};

#[derive(Debug)]
#[non_exhaustive]
pub struct Request {
    pub(super) route: Route,
    pub(super) body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(route: Route) -> Self {
        Self { route, body: None }
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn to_hyper_request(self, token: &str) -> HyperRequest<Body> {
        let Self { route, body, .. } = self;
        let (method, path) = route.method_and_path();

        let mut builder = HyperRequest::builder();

        if body.is_some() {
            builder = builder.header(CONTENT_TYPE, "application/json");
        }

        builder
            .method(method)
            .uri(format!("https://discord.com/api/v9{}", path))
            .header(AUTHORIZATION, format!("Bot {}", token))
            .header(USER_AGENT, r#"DiscordBot ("", "0.1.0")"#)
            .body(body.map_or(Body::empty(), Body::from))
            .expect("Error building hyper request")
    }
}
