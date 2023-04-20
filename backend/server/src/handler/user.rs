use axum::{async_trait, Json};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use tracing::info;
use uchat_domain::{ids::*, user::DisplayName};
use uchat_endpoint::{
    user::{
        endpoint::{
            CreateUser, CreateUserOk, FollowUser, FollowUserOk, GetMyProfile, GetMyProfileOk,
            Login, LoginOk, UpdateProfile, UpdateProfileOk, ViewProfile, ViewProfileOk,
        },
        types::{FollowAction, PublicUserProfile},
    },
    RequestFailed, Update,
};
use uchat_query::{
    session::Session,
    user::{UpdateProfileParams, User},
    AsyncConnection,
};
use url::Url;

use crate::{
    error::{ApiError, ApiResult},
    extractor::{DbConnection, UserSession},
    AppState,
};

use super::{save_image, AuthorizedApiRequest, PublicApiRequest};

fn profile_id_to_url(id: &str) -> Url {
    use uchat_endpoint::app_url::{self, user_content};
    app_url::domain_and(user_content::ROOT)
        .join(user_content::IMAGES)
        .unwrap()
        .join(id)
        .unwrap()
}

#[derive(Clone)]
pub struct SessionSignature(String);

pub fn to_public(
    conn: &mut uchat_query::AsyncConnection,
    session: Option<&UserSession>,
    user: User,
) -> ApiResult<PublicUserProfile> {
    Ok(PublicUserProfile {
        id: user.id,
        display_name: user
            .display_name
            .and_then(|name| DisplayName::new(name).ok()),
        handle: user.handle,
        profile_image: user.profile_image.as_ref().map(|id| profile_id_to_url(id)),
        created_at: user.created_at,
        am_following: {
            match session {
                Some(session) => uchat_query::user::is_following(conn, session.user_id, user.id)?,
                None => false,
            }
        },
    })
}

fn new_session(
    state: &AppState,
    conn: &mut uchat_query::AsyncConnection,
    user_id: UserId,
) -> ApiResult<(Session, SessionSignature, Duration)> {
    let fingerprint = serde_json::json!({});
    let session_duration = Duration::weeks(3);
    let session = uchat_query::session::new(conn, user_id, session_duration, fingerprint.into())?;

    let mut rng = state.rng.clone();
    let signature = state
        .signing_keys
        .sign(&mut rng, session.id.as_uuid().as_bytes());

    let signature = uchat_crypto::encode_base64(signature);
    Ok((session, SessionSignature(signature), session_duration))
}

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = uchat_crypto::hash_password(&self.password)?;
        let user_id = uchat_query::user::new(&mut conn, password_hash, &self.username)?;

        info!(username = self.username.as_ref(), "new user created");

        let (session, signature, duration) = new_session(&state, &mut conn, user_id)?;

        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                user_id,
                username: self.username,
                session_signature: signature.0,
                session_id: session.id,
                session_expires: Utc::now() + duration,
            }),
        ))
    }
}

#[async_trait]
impl PublicApiRequest for Login {
    type Response = (StatusCode, Json<LoginOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let _span = tracing::span!(tracing::Level::INFO, "logging in",
                user = %self.username.as_ref())
        .entered();
        let hash = uchat_query::user::get_password_hash(&mut conn, &self.username)?;
        let hash = uchat_crypto::password::deserialize_hash(&hash)?;

        uchat_crypto::verify_password(self.password, &hash)?;

        let user = uchat_query::user::find(&mut conn, &self.username)?;

        let (session, signature, duration) = new_session(&state, &mut conn, user.id)?;

        let profile_image_url = user.profile_image.as_ref().map(|id| profile_id_to_url(id));

        Ok((
            StatusCode::OK,
            Json(LoginOk {
                session_id: session.id,
                session_expires: Utc::now() + duration,
                session_signature: signature.0,
                display_name: user.display_name,
                email: user.email,
                profile_image: profile_image_url,
                user_id: user.id,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for GetMyProfile {
    type Response = (StatusCode, Json<GetMyProfileOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let user = uchat_query::user::get(&mut conn, session.user_id)?;

        let profile_image_url = user.profile_image.as_ref().map(|id| profile_id_to_url(id));

        Ok((
            StatusCode::OK,
            Json(GetMyProfileOk {
                display_name: user.display_name,
                email: user.email,
                profile_image: profile_image_url,
                user_id: user.id,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for UpdateProfile {
    type Response = (StatusCode, Json<UpdateProfileOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let mut payload = self;
        let password = {
            if let Update::Change(ref password) = payload.password {
                Update::Change(uchat_crypto::hash_password(password)?)
            } else {
                Update::NoChange
            }
        };

        if let Update::Change(ref img) = payload.profile_image {
            let id = ImageId::new();
            save_image(id, img).await?;
            payload.profile_image = Update::Change(id.to_string());
        }

        let query_params = UpdateProfileParams {
            id: session.user_id,
            display_name: payload.display_name,
            email: payload.email,
            password_hash: password,
            profile_image: payload.profile_image.clone(),
        };

        uchat_query::user::update_profile(&mut conn, query_params)?;

        let profile_image_url = {
            let user = uchat_query::user::get(&mut conn, session.user_id)?;
            user.profile_image.as_ref().map(|id| profile_id_to_url(id))
        };

        Ok((
            StatusCode::OK,
            Json(UpdateProfileOk {
                profile_image: profile_image_url,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for ViewProfile {
    type Response = (StatusCode, Json<ViewProfileOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let profile = uchat_query::user::get(&mut conn, self.for_user)?;
        let profile = to_public(&mut conn, Some(&session), profile)?;

        let mut posts = vec![];

        for post in uchat_query::post::get_public_posts(&mut conn, self.for_user)? {
            let post_id = post.id;
            match super::post::to_public(&mut conn, post, Some(&session)) {
                Ok(post) => posts.push(post),
                Err(e) => {
                    tracing::error!(err = %e.err, post_id = ?post_id, "post contains invalid data");
                }
            }
        }

        Ok((StatusCode::OK, Json(ViewProfileOk { profile, posts })))
    }
}

#[async_trait]
impl AuthorizedApiRequest for FollowUser {
    type Response = (StatusCode, Json<FollowUserOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        if self.user_id == session.user_id {
            return Err(ApiError {
                code: Some(StatusCode::BAD_REQUEST),
                err: color_eyre::Report::new(RequestFailed {
                    msg: "cannot follow self".to_string(),
                }),
            });
        }
        match self.action {
            FollowAction::Follow => {
                uchat_query::user::follow(&mut conn, session.user_id, self.user_id)?;
            }
            FollowAction::Unfollow => {
                uchat_query::user::unfollow(&mut conn, session.user_id, self.user_id)?;
            }
        }

        Ok((
            StatusCode::OK,
            Json(FollowUserOk {
                status: self.action,
            }),
        ))
    }
}
