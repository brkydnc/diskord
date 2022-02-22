use super::request::Request;
use crate::snowflake::Snowflake;
use hyper::Method;
use serde_json::json;
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Route {
    GetCurrentUser,
    GetUser {
        user_id: Snowflake,
    },
    // All parameters are optional for this endpoint. PROBABLY (not tested)
    // setting avatar as null removes user's avatar. Thus, when serializing,
    // it is not clear whether using `None` means nullable or optional.
    //
    // TODO: Use enum to serialize without confusion.
    //
    // ModifyCurrentUser {
    //     username: Option<&'a str>,
    //     avatar: Option<&'a str>
    // },
    GetCurrentUserGuilds {
        before: Option<Snowflake>,
        after: Option<Snowflake>,
        limit: Option<NonZeroU8>,
    },
    GetCurrentUserGuildMember {
        guild_id: Snowflake,
    },
    LeaveGuild {
        guild_id: Snowflake,
    },
    CreateDM {
        recipient_id: Snowflake,
    },
    GetUserConnections,
}

impl Route {
    pub fn method_and_path(&self) -> (Method, String) {
        use Route::*;

        match self {
            GetCurrentUser => (Method::GET, "/users/@me".into()),
            GetUser { user_id } => (Method::GET, format!("/users/{}", user_id)),
            GetCurrentUserGuilds {
                before,
                after,
                limit,
            } => {
                let mut path = String::from("/users/@me/guilds?");

                if let Some(before) = before {
                    path.push_str(&format!("&before={}", before));
                }

                if let Some(after) = after {
                    path.push_str(&format!("&after={}", after));
                }

                if let Some(limit) = limit {
                    path.push_str(&format!("&limit={}", limit));
                }

                (Method::GET, path)
            }
            GetCurrentUserGuildMember { guild_id } => (
                Method::GET,
                format!("/users/@me/guilds/{}/member", guild_id),
            ),
            LeaveGuild { guild_id } => (Method::DELETE, format!("/users/@me/guilds/{}", guild_id)),
            CreateDM { .. } => (Method::POST, "/users/@me/channels".into()),
            GetUserConnections => (Method::GET, "/users/@me/connections".into()),
        }
    }
}

impl From<Route> for Request {
    fn from(route: Route) -> Request {
        use Route::*;

        match route {
            route @ CreateDM { recipient_id } => {
                let value = json!({ "recipient_id": recipient_id });
                let body = serde_json::to_vec(&value).unwrap();
                Request::new(route).with_body(body)
            }
            route => Request::new(route),
        }
    }
}
