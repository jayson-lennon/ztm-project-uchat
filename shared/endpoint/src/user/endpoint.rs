use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uchat_domain::{ids::*, Password, Username};
use url::Url;

use crate::Endpoint;

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: Username,
    pub password: Password,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUserOk {
    pub user_id: UserId,
    pub username: Username,

    pub session_signature: String,
    pub session_id: SessionId,
    pub session_expires: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Login {
    pub username: Username,
    pub password: Password,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LoginOk {
    pub session_signature: String,
    pub session_id: SessionId,
    pub session_expires: DateTime<Utc>,

    pub display_name: Option<String>,
    pub email: Option<String>,
    pub profile_image: Option<Url>,
    pub user_id: UserId,
}
