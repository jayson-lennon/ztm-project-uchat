use axum::{async_trait, Json};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use tracing::info;
use uchat_domain::ids::*;
use uchat_endpoint::user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk};
use uchat_query::session::Session;

use crate::{error::ApiResult, extractor::DbConnection, AppState};

use super::PublicApiRequest;

#[derive(Clone)]
pub struct SessionSignature(String);

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

        Ok((
            StatusCode::OK,
            Json(LoginOk {
                session_id: session.id,
                session_expires: Utc::now() + duration,
                session_signature: signature.0,
                display_name: user.display_name,
                email: user.email,
                profile_image: None,
                user_id: user.id,
            }),
        ))
    }
}
