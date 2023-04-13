use axum::{async_trait, Json};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use tracing::info;
use uchat_domain::{ids::*, Username};
use uchat_endpoint::{
    post::{
        endpoint::{
            Bookmark, BookmarkOk, Boost, BoostOk, NewPost, NewPostOk, React, ReactOk,
            TrendingPosts, TrendingPostsOk,
        },
        types::{BookmarkAction, BoostAction, LikeStatus, PublicPost},
    },
    user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk},
    RequestFailed,
};
use uchat_query::{post::Post, session::Session, AsyncConnection};

use crate::{
    error::{ApiError, ApiResult},
    extractor::{DbConnection, UserSession},
    AppState,
};

use super::AuthorizedApiRequest;

pub fn to_public(
    conn: &mut AsyncConnection,
    post: Post,
    session: Option<&UserSession>,
) -> ApiResult<PublicPost> {
    use uchat_query::post as query_post;
    use uchat_query::user as query_user;

    if let Ok(mut content) = serde_json::from_value(post.content.0) {
        let aggregate_reactions = query_post::aggregate_reactions(conn, post.id)?;
        Ok(PublicPost {
            id: post.id,
            by_user: {
                let profile = query_user::get(conn, post.user_id)?;
                super::user::to_public(profile)?
            },
            content,
            time_posted: post.time_posted,
            reply_to: {
                match post.reply_to {
                    Some(other_post_id) => {
                        let original_post = query_post::get(conn, other_post_id)?;
                        let original_user = query_user::get(conn, original_post.user_id)?;
                        Some((
                            Username::new(original_user.handle).unwrap(),
                            original_user.id,
                            other_post_id,
                        ))
                    }
                    None => None,
                }
            },
            like_status: {
                match session {
                    Some(session) => {
                        match query_post::get_reaction(conn, post.id, session.user_id)? {
                            Some(reaction) if reaction.like_status == -1 => LikeStatus::Dislike,
                            Some(reaction) if reaction.like_status == 1 => LikeStatus::Like,
                            _ => LikeStatus::NoReaction,
                        }
                    }
                    None => LikeStatus::NoReaction,
                }
            },
            bookmarked: {
                match session {
                    Some(session) => query_post::get_bookmark(conn, session.user_id, post.id)?,
                    None => false,
                }
            },
            boosted: {
                match session {
                    Some(session) => query_post::get_boost(conn, session.user_id, post.id)?,
                    None => false,
                }
            },
            likes: aggregate_reactions.likes,
            dislikes: aggregate_reactions.dislikes,
            boosts: aggregate_reactions.boosts,
        })
    } else {
        Err(ApiError {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR),
            err: color_eyre::Report::new(RequestFailed {
                msg: "invalid post data".to_string(),
            }),
        })
    }
}

#[async_trait]
impl AuthorizedApiRequest for NewPost {
    type Response = (StatusCode, Json<NewPostOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let post = Post::new(session.user_id, self.content, self.options)?;

        let post_id = uchat_query::post::new(&mut conn, post)?;

        Ok((StatusCode::OK, Json(NewPostOk { post_id })))
    }
}

#[async_trait]
impl AuthorizedApiRequest for TrendingPosts {
    type Response = (StatusCode, Json<TrendingPostsOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        use uchat_query::post as query_post;

        let mut posts = vec![];

        for post in query_post::get_trending(&mut conn)? {
            let post_id = post.id;
            match to_public(&mut conn, post, Some(&session)) {
                Ok(post) => posts.push(post),
                Err(e) => {
                    tracing::error!(err = %e.err, post_id = ?post_id, "post contains invalid data");
                }
            }
        }

        Ok((StatusCode::OK, Json(TrendingPostsOk { posts })))
    }
}

#[async_trait]
impl AuthorizedApiRequest for Bookmark {
    type Response = (StatusCode, Json<BookmarkOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        match self.action {
            BookmarkAction::Add => {
                uchat_query::post::bookmark(&mut conn, session.user_id, self.post_id)?;
            }
            BookmarkAction::Remove => {
                uchat_query::post::delete_bookmark(&mut conn, session.user_id, self.post_id)?;
            }
        }

        Ok((
            StatusCode::OK,
            Json(BookmarkOk {
                status: self.action,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for React {
    type Response = (StatusCode, Json<ReactOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        use uchat_endpoint::post::types::LikeStatus;

        let reaction = uchat_query::post::Reaction {
            post_id: self.post_id,
            user_id: session.user_id,
            reaction: None,
            like_status: match self.like_status {
                LikeStatus::Like => 1,
                LikeStatus::Dislike => -1,
                LikeStatus::NoReaction => 0,
            },
            created_at: Utc::now(),
        };

        uchat_query::post::react(&mut conn, reaction)?;

        let aggregate_reactions = uchat_query::post::aggregate_reactions(&mut conn, self.post_id)?;

        Ok((
            StatusCode::OK,
            Json(ReactOk {
                like_status: self.like_status,
                likes: aggregate_reactions.likes,
                dislikes: aggregate_reactions.dislikes,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for Boost {
    type Response = (StatusCode, Json<BoostOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        match self.action {
            BoostAction::Add => {
                uchat_query::post::boost(&mut conn, session.user_id, self.post_id, Utc::now())?;
            }
            BoostAction::Remove => {
                uchat_query::post::delete_boost(&mut conn, session.user_id, self.post_id)?;
            }
        }

        Ok((
            StatusCode::OK,
            Json(BoostOk {
                status: self.action,
            }),
        ))
    }
}
