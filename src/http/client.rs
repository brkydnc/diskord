use super::{error::ApiError, request::Request, route::Route, util::deserialize_response};
use crate::error::Result;
use crate::model::user::{Connection, User};
use hyper::{body::Body, client::HttpConnector, Client as HyperClient};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

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

    async fn request<T>(&self, route: Route) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = Request::from(route).to_hyper_request(&self.token);
        let response = self.client.request(request).await?;

        if response.status().is_success() {
            deserialize_response(response).await
        } else {
            Err(ApiError::try_from_response(response).await?.into())
        }
    }

    async fn request_ignore_response(&self, route: Route) -> Result<()> {
        let request = Request::from(route).to_hyper_request(&self.token);
        let response = self.client.request(request).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(ApiError::try_from_response(response).await?.into())
        }
    }

    pub async fn get_current_user(&self) -> Result<User> {
        self.request(Route::GetCurrentUser).await
    }

    pub async fn get_user(&self, id: u64) -> Result<User> {
        let user_id = id.try_into()?;
        let route = Route::GetUser { user_id };
        self.request(route).await
    }

    pub async fn leave_guild(&self, id: u64) -> Result<()> {
        let guild_id = id.try_into()?;
        let route = Route::LeaveGuild { guild_id };
        self.request_ignore_response(route).await
    }

    pub async fn get_user_connections(&self) -> Result<Vec<Connection>> {
        self.request(Route::GetUserConnections).await
    }
}
