use serde::{Deserialize, Serialize};
use uchat_domain::{ids::*, Password, Username};

use crate::Endpoint;

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: Username,
    pub password: Password,
}

impl Endpoint for CreateUser {
    const URL: &'static str = "/account/create";
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUserOk {
    pub user_id: UserId,
    pub username: Username,
}
