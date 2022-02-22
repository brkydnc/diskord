use super::route::Route;
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Body, Request as HyperRequest,
};

#[derive(Debug)]
#[non_exhaustive]
pub struct Request {
    route: Route,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn to_hyper_request(self, token: &str) -> HyperRequest<Body> {
        let Self { route, body, .. } = self;
        let (method, path) = route.method_and_path();
        let body = body.map_or(Body::empty(), |body| Body::from(body));

        HyperRequest::builder()
            .method(method)
            .uri(format!("https://discord.com/api/v9{}", path))
            .header(AUTHORIZATION, format!("Bot {}", token))
            .header(USER_AGENT, r#"DiscordBot ("", "0.1.0")"#)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .expect("Error building hyper request")
    }
}
