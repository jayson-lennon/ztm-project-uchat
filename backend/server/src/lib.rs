use axum::extract::FromRef;
use uchat_query::{AsyncConnection, AsyncConnectionPool, QueryError};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: AsyncConnectionPool,
    pub signing_key: uchat_crypto::sign::Keys,
    pub rng: rand::rngs::StdRng,
}

impl AppState {
    pub async fn connect(&self) -> Result<AsyncConnection, QueryError> {
        self.db_pool.get().await
    }
}

pub mod cli {
    use rand_core::{CryptoRng, RngCore};
    use uchat_crypto::sign::{EncodedPrivateKey, Keys};

    pub fn gen_keys<R>(rng: &mut R) -> color_eyre::Result<(EncodedPrivateKey, Keys)>
    where
        R: CryptoRng + RngCore,
    {
        let (private_key, public_key) = Keys::generate(rng)?;
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
