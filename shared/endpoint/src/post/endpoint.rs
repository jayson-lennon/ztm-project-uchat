use serde::{Deserialize, Serialize};
use uchat_domain::ids::{PollChoiceId, PostId};



use super::types::{
    BookmarkAction, BoostAction, Content, LikeStatus, NewPostOptions, PublicPost, VoteCast,
};

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bookmark {
    pub post_id: PostId,
    pub action: BookmarkAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookmarkOk {
    pub status: BookmarkAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct React {
    pub post_id: PostId,
    pub like_status: LikeStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ReactOk {
    pub like_status: LikeStatus,
    pub likes: i64,
    pub dislikes: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Boost {
    pub post_id: PostId,
    pub action: BoostAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BoostOk {
    pub status: BoostAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Vote {
    pub post_id: PostId,
    pub choice_id: PollChoiceId,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct VoteOk {
    pub cast: VoteCast,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HomePosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HomePostsOk {
    pub posts: Vec<PublicPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LikedPosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LikedPostsOk {
    pub posts: Vec<PublicPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookmarkedPosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookmarkedPostsOk {
    pub posts: Vec<PublicPost>,
}
