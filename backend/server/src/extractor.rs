use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, Extension, RequestPartsExt,
};
use hyper::StatusCode;
use uchat_query::OwnedAsyncConnection;

use crate::AppState;

pub struct DbConnection(pub OwnedAsyncConnection);

#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap();
        let connection = state.db_pool.get_owned().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to connect to database",
            )
        })?;
        Ok(Self(connection))
    }
}
