use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uchat_domain::{ids::*, Password, Username};
use url::Url;

use crate::{post::types::PublicPost, Endpoint, Update};

use super::types::PublicUserProfile;

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

#[derive(Clone, Deserialize, Serialize)]
pub struct GetMyProfile;

#[derive(Clone, Deserialize, Serialize)]
pub struct GetMyProfileOk {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub profile_image: Option<Url>,
    pub user_id: UserId,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateProfile {
    pub display_name: Update<String>,
    pub email: Update<String>,
    pub profile_image: Update<String>,
    pub password: Update<Password>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateProfileOk {
    pub profile_image: Option<Url>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ViewProfile {
    pub for_user: UserId,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ViewProfileOk {
    pub profile: PublicUserProfile,
    pub posts: Vec<PublicPost>,
}
