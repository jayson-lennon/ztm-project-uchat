use diesel::{Connection, PgConnection, RunQueryDsl};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn reset_database(connection_url: &str) {
    // connect to "postgres" database
    let (database, postgres_url) = query_helper::change_database_of_url(connection_url, "postgres");

    let mut conn = PgConnection::establish(&postgres_url).unwrap();

    // make the test database
    if let Err(e) = query_helper::create_database(&database).execute(&mut conn) {
        eprintln!("database creation error: {e}");
    }
}

/// Create a new test connection. Requires `TEST_DATABASE_URL` environment variable set.
///
/// Data committed on this connection will not get saved to the database.
pub fn new_connection() -> PgConnection {
    // https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/schema/mod.rs
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use std::sync::Once;
    static MIGRATION_GUARD: Once = Once::new();

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

    let mut conn = {
        let connection_url = dotenvy::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set in order to run tests");

        // make new database if needed
        if PgConnection::establish(&connection_url).is_err() {
            reset_database(&connection_url);
        }

        // connect to the test database
        PgConnection::establish(&connection_url).unwrap()
    };

    // run migrations if needed
    MIGRATION_GUARD.call_once(|| {
        if let Err(e) = conn.run_pending_migrations(MIGRATIONS) {
            use diesel::result::DatabaseErrorKind;
            use diesel::result::Error as DieselError;
            match *e.downcast::<DieselError>().unwrap() {
                DieselError::DatabaseError(kind, _) => match kind {
                    // When multiple threads try to create the database (and it doesn't exist)
                    // then we may get some UniqueViolations as the threads try to create the
                    // database at the same time. Ignore this error and report all others.
                    DatabaseErrorKind::UniqueViolation => (),
                    other => eprintln!("database creation error: {other:?}"),
                },
                other => eprintln!("database creation error: {other}"),
            }
        }
    });

    // test transactions are never committed
    conn.begin_test_transaction().unwrap();
    conn
}

/// Queries for creating and dropping databases.
// https://github.com/diesel-rs/diesel/blob/master/diesel_cli/src/query_helper.rs
mod query_helper {
    use diesel::{
        backend::Backend,
        query_builder::{AstPass, QueryFragment, QueryId},
        QueryResult, RunQueryDsl,
    };

    #[derive(Debug, Clone)]
    pub struct DropDatabaseStatement {
        db_name: String,
        if_exists: bool,
    }

    impl DropDatabaseStatement {
        #[allow(dead_code)]
        pub fn new(db_name: &str) -> Self {
            DropDatabaseStatement {
                db_name: db_name.to_owned(),
                if_exists: false,
            }
        }

        #[allow(dead_code)]
        pub fn if_exists(self) -> Self {
            DropDatabaseStatement {
                if_exists: true,
                ..self
            }
        }
    }

    impl<DB: Backend> QueryFragment<DB> for DropDatabaseStatement {
        fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
            out.push_sql("DROP DATABASE ");
            if self.if_exists {
                out.push_sql("IF EXISTS ");
            }
            out.push_identifier(&self.db_name)?;
            Ok(())
        }
    }

    impl<Conn> RunQueryDsl<Conn> for DropDatabaseStatement {}

    impl QueryId for DropDatabaseStatement {
        type QueryId = ();

        const HAS_STATIC_QUERY_ID: bool = false;
    }

    #[allow(dead_code)]
    pub fn drop_database(db_name: &str) -> DropDatabaseStatement {
        DropDatabaseStatement::new(db_name).if_exists()
    }

    #[derive(Debug, Clone)]
    pub struct CreateDatabaseStatement {
        db_name: String,
    }

    impl CreateDatabaseStatement {
        pub fn new(db_name: &str) -> Self {
            CreateDatabaseStatement {
                db_name: db_name.to_owned(),
            }
        }
    }

    impl<DB: Backend> QueryFragment<DB> for CreateDatabaseStatement {
        fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
            out.push_sql("CREATE DATABASE ");
            out.push_identifier(&self.db_name)?;
            Ok(())
        }
    }

    impl<Conn> RunQueryDsl<Conn> for CreateDatabaseStatement {}

    impl QueryId for CreateDatabaseStatement {
        type QueryId = ();

        const HAS_STATIC_QUERY_ID: bool = false;
    }

    pub fn create_database(db_name: &str) -> CreateDatabaseStatement {
        CreateDatabaseStatement::new(db_name)
    }

    pub fn change_database_of_url(database_url: &str, default_database: &str) -> (String, String) {
        let base = ::url::Url::parse(database_url).unwrap();
        let database = base.path_segments().unwrap().last().unwrap().to_owned();
        let mut new_url = base.join(default_database).unwrap();
        new_url.set_query(base.query());
        (database, new_url.into())
    }
}
