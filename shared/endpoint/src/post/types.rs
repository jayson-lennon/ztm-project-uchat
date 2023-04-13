use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uchat_domain::{
    ids::{PostId, UserId},
    post::{Headline, Message},
    Username,
};

use crate::user::types::PublicUserProfile;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Chat {
    pub headline: Option<Headline>,
    pub message: Message,
}

impl From<Chat> for Content {
    fn from(value: Chat) -> Self {
        Content::Chat(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Content {
    Chat(Chat),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPostOptions {
    pub reply_to: Option<PostId>,
    pub direct_message_to: Option<UserId>,
    pub time_posted: DateTime<Utc>,
}

impl Default for NewPostOptions {
    fn default() -> Self {
        Self {
            reply_to: None,
            direct_message_to: None,
            time_posted: Utc::now(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum LikeStatus {
    Dislike,
    Like,
    NoReaction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PublicPost {
    pub id: PostId,
    pub by_user: PublicUserProfile,
    pub content: Content,
    pub time_posted: DateTime<Utc>,
    pub reply_to: Option<(Username, UserId, PostId)>,
    pub like_status: LikeStatus,
    pub bookmarked: bool,
    pub boosted: bool,
    pub likes: i64,
    pub dislikes: i64,
    pub boosts: i64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum BookmarkAction {
    Add,
    Remove,
}

impl From<BookmarkAction> for bool {
    fn from(value: BookmarkAction) -> Self {
        match value {
            BookmarkAction::Add => true,
            BookmarkAction::Remove => false,
        }
    }
}
