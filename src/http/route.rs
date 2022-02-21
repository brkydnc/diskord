use crate::snowflake::Snowflake;
use std::num::NonZeroU8;

#[derive(Debug)]
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
    pub fn to_path(&self) -> String {
        use Route::*;
        match self {
            GetCurrentUser => "/users/@me".into(),
            GetUser { user_id } => format!("/users/{}", user_id),
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

                path
            }
            GetCurrentUserGuildMember { guild_id } => {
                format!("/users/@me/guilds/{}/member", guild_id)
            }
            LeaveGuild { guild_id } => format!("/users/@me/guilds/{}", guild_id),
            CreateDM { .. } => "/users/@me/channels".into(),
            GetUserConnections => "/users/@me/connections".into(),
        }
    }
}
