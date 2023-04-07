use diesel::prelude::*;
use diesel::PgConnection;
use password_hash::PasswordHashString;
use uchat_domain::ids::UserId;

use crate::QueryError;

pub fn new<T: AsRef<str>>(
    conn: &mut PgConnection,
    hash: PasswordHashString,
    handle: T,
) -> Result<UserId, QueryError> {
    use crate::schema::users::{self, columns};

    let user_id = UserId::new();

    diesel::insert_into(users::table)
        .values((
            columns::id.eq(user_id),
            columns::password_hash.eq(hash.as_str()),
            columns::handle.eq(handle.as_ref()),
        ))
        .execute(conn)?;

    Ok(user_id)
}
