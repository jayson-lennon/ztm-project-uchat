use diesel::{ConnectionError, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{error::Error, time::Duration};

use crate::error::QueryError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

type AsyncConnectionManager = bb8_diesel::DieselConnectionManager<PgConnection>;

pub type AsyncConnection<'a> = bb8::PooledConnection<'a, AsyncConnectionManager>;
pub type OwnedAsyncConnection = bb8::PooledConnection<'static, AsyncConnectionManager>;

#[derive(Clone, Debug)]
pub struct AsyncConnectionPool(bb8::Pool<AsyncConnectionManager>);

impl AsyncConnectionPool {
    pub async fn new<S: AsRef<str>>(url: S) -> Result<Self, QueryError> {
        let pool = new_async_pool(url).await?;
        {
            // check connection
            let _ = pool
                .0
                .get()
                .await
                .map_err(|e| QueryError::Connection(e.to_string()))?;
        }
        Ok(pool)
    }

    pub async fn get(&self) -> Result<AsyncConnection, QueryError> {
        self.0
            .get()
            .await
            .map_err(|e| QueryError::Connection(e.to_string()))
    }

    pub async fn get_owned(&self) -> Result<OwnedAsyncConnection, QueryError> {
        self.0
            .get_owned()
            .await
            .map_err(|e| QueryError::Connection(e.to_string()))
    }

    pub fn state(&self) -> bb8::State {
        self.0.state()
    }
}

/// Run database migrations
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

/// Connect to the database
pub fn connect<S: AsRef<str>>(url: S) -> Result<PgConnection, ConnectionError> {
    use diesel::Connection;
    let url = url.as_ref();
    PgConnection::establish(url)
}

/// Usage:
/// ```ignore
/// let async_pool = new_async_pool("postgres://login@localhost/sample").await;
/// let conn = &mut async_pool.get().await?;
/// ```
pub async fn new_async_pool<S: AsRef<str>>(url: S) -> Result<AsyncConnectionPool, QueryError> {
    let url = url.as_ref();
    let manager = bb8_diesel::DieselConnectionManager::<PgConnection>::new(url);
    bb8::Pool::builder()
        .test_on_check_out(true)
        .connection_timeout(Duration::from_secs(10))
        .build(manager)
        .await
        .map(AsyncConnectionPool)
        .map_err(|e| QueryError::Pool(e.to_string()))
}
