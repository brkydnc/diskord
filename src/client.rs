use hyper::{
    client::{HttpConnector, ResponseFuture},
    Body, Client as HyperClient, Method, Request,
};
use hyper_tls::HttpsConnector;

pub struct Client {
    http: HyperClient<HttpsConnector<HttpConnector>>,
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        let https = HttpsConnector::new();
        let hyper_client = HyperClient::builder().build::<_, Body>(https);

        Self {
            http: hyper_client,
            token: token.to_string(),
        }
    }

    fn request(&self, method: Method, path: &str) -> ResponseFuture {
        let request = Request::builder()
            .method(method)
            .header("Authorization", format!("Bot {}", self.token))
            .uri(format!("https://discord.com/api/v9{}", path))
            .body(Body::empty())
            .expect("Error building request");

        self.http.request(request)
    }

    pub fn get_user(&self, id: &str) -> ResponseFuture {
        self.request(Method::GET, &format!("/users/{}", id))
    }

    pub fn get_guild_member(&self, guild_id: &str, user_id: &str) -> ResponseFuture {
        self.request(
            Method::GET,
            &format!("/guilds/{}/members/{}", guild_id, user_id),
        )
    }
}
