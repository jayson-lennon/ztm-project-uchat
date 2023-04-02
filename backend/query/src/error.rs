pub use diesel::result::Error as DieselError;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("database connection pool error: {0}")]
    Pool(String),

    #[error("database connection error: {0}")]
    Connection(String),

    #[error("database error: {0}")]
    Database(DieselError),

    #[error("unique violation")]
    UniqueViolation,

    #[error("foreign key violation")]
    ForeignKeyViolation,

    #[error("check constraint violation")]
    CheckViolation,

    #[error("not found")]
    NotFound,
}

impl From<DieselError> for QueryError {
    fn from(e: DieselError) -> Self {
        use diesel::result::DatabaseErrorKind::*;
        use DieselError::DatabaseError;

        match e {
            DatabaseError(UniqueViolation, _) => Self::UniqueViolation,
            DatabaseError(ForeignKeyViolation, _) => Self::ForeignKeyViolation,
            DatabaseError(CheckViolation, _) => Self::CheckViolation,
            DieselError::NotFound => Self::NotFound,
            other => Self::Database(other),
        }
    }
}
