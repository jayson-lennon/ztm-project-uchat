use serde::{Deserialize, Serialize};
use uchat_domain::ids::PostId;

use crate::Endpoint;

use super::types::{Content, NewPostOptions, PublicPost};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPost {
    pub content: Content,
    pub options: NewPostOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPostOk {
    pub post_id: PostId,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrendingPosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrendingPostsOk {
    pub posts: Vec<PublicPost>,
}
