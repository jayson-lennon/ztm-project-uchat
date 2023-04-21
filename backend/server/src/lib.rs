use axum::extract::FromRef;
use uchat_query::{AsyncConnection, AsyncConnectionPool, QueryError};

pub mod error;
pub mod extractor;
pub mod handler;
pub mod logging;
pub mod router;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db_pool: AsyncConnectionPool,
    pub signing_keys: uchat_crypto::sign::Keys,
    pub rng: rand::rngs::StdRng,
}

impl AppState {
    pub async fn connect(&self) -> Result<AsyncConnection, QueryError> {
        self.db_pool.get().await
    }
}

pub mod cli {
    use color_eyre::{eyre::Context, Help};
    use rand::{CryptoRng, RngCore};
    use uchat_crypto::sign::{encode_private_key, EncodedPrivateKey, Keys};

    pub fn gen_keys<R>(rng: &mut R) -> color_eyre::Result<(EncodedPrivateKey, Keys)>
    where
        R: CryptoRng + RngCore,
    {
        let (private_key, keys) = Keys::generate(rng)?;
        let private_key = encode_private_key(private_key)?;
        Ok((private_key, keys))
    }

    pub fn load_keys() -> color_eyre::Result<Keys> {
        let private_key = std::env::var("API_PRIVATE_KEY")
            .wrap_err("failed to locate private API key")
            .suggestion("set API_PRIVATE_KEY environment variable")?;

        Ok(Keys::from_encoded(private_key)?)
    }
}

#[cfg(test)]
pub mod tests {
    use hyper::StatusCode;
    use uchat_domain::{Password, Username};
    use uchat_endpoint::{
        user::endpoint::{CreateUser, CreateUserOk},
        Endpoint,
    };

    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub mod util {
        use axum::{
            response::{IntoResponse, Response},
            Router,
        };
        use hyper::Request;
        use serde::Serialize;
        use tower::ServiceExt;
        use uchat_crypto::sign::Keys;
        use uchat_query::AsyncConnectionPool;

        use crate::AppState;

        pub async fn new_state() -> AppState {
            let connection_url = dotenvy::var("TEST_DATABASE_URL")
                .expect("TEST_DATABASE_URL must be set in order to run tests");
            let mut rng = uchat_crypto::new_rng();
            AppState {
                db_pool: AsyncConnectionPool::new(connection_url).await.unwrap(),
                signing_keys: Keys::generate(&mut rng).unwrap().1,
                rng,
            }
        }

        pub async fn new_router() -> Router {
            let state = new_state().await;
            crate::router::new_router(state)
        }

        pub async fn api_request_with_router<P>(router: Router, uri: &str, payload: P) -> Response
        where
            P: Serialize,
        {
            let payload = serde_json::to_string(&payload).unwrap();
            router
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .header("Content-Type", "application/json")
                        .uri(uri)
                        .body(payload.into())
                        .unwrap(),
                )
                .await
                .unwrap()
                .into_response()
        }

        pub async fn api_request<P>(uri: &str, payload: P) -> Response
        where
            P: Serialize,
        {
            let router = new_router().await;
            api_request_with_router(router, uri, payload).await
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn create_user() -> Result<()> {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let username: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        // user doesnt exist yet
        {
            let payload = CreateUser {
                password: Password::new("password")?,
                username: Username::new(&username)?,
            };

            let response = util::api_request(CreateUser::URL, payload).await;

            assert_eq!(StatusCode::CREATED, response.status());

            let response = hyper::body::to_bytes(response.into_body()).await?;
            let response: CreateUserOk = serde_json::from_slice(&response)?;

            assert_eq!(username, response.username.into_inner());
        }

        // try to add duplicate user
        {
            let payload = CreateUser {
                password: Password::new("password")?,
                username: Username::new(username)?,
            };
            let response = util::api_request(CreateUser::URL, payload).await;

            assert_eq!(StatusCode::CONFLICT, response.status());
        }

        Ok(())
    }
}
