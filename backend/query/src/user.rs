use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use password_hash::PasswordHashString;
use uchat_domain::ids::UserId;
use uchat_domain::Username;
use uchat_endpoint::Update;

use crate::post::DeleteStatus;
use crate::{DieselError, QueryError};

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

pub fn get_password_hash(
    conn: &mut PgConnection,
    username: &Username,
) -> Result<String, QueryError> {
    use crate::schema::users::dsl::*;
    Ok(users
        .filter(handle.eq(username.as_ref()))
        .select(password_hash)
        .get_result(conn)?)
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: UserId,
    pub email: Option<String>,
    pub email_confirmed: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub handle: String,
    pub created_at: DateTime<Utc>,
    pub profile_image: Option<String>,
}

pub fn get(conn: &mut PgConnection, user_id: UserId) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;
    users.filter(id.eq(user_id)).get_result(conn)
}

pub fn find(conn: &mut PgConnection, username: &Username) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;
    users.filter(handle.eq(username.as_ref())).get_result(conn)
}

#[derive(Debug)]
pub struct UpdateProfileParams {
    pub id: UserId,
    pub display_name: Update<String>,
    pub email: Update<String>,
    pub password_hash: Update<PasswordHashString>,
    pub profile_image: Update<String>,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = crate::schema::users)]
struct UpdateProfileParamsInternal {
    pub display_name: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub password_hash: Option<String>,
    pub profile_image: Option<Option<String>>,
}

pub fn update_profile(
    conn: &mut PgConnection,
    query_params: UpdateProfileParams,
) -> Result<(), DieselError> {
    use crate::schema::users;

    let update = UpdateProfileParamsInternal {
        display_name: query_params.display_name.into_nullable(),
        email: query_params.email.into_nullable(),
        password_hash: query_params
            .password_hash
            .into_option()
            .map(|s| s.to_string()),
        profile_image: query_params.profile_image.into_nullable(),
    };

    diesel::update(users::table)
        .filter(users::id.eq(&query_params.id))
        .set(&update)
        .execute(conn)
        .map(|_| ())
}

pub fn follow(conn: &mut PgConnection, user_id: UserId, follow: UserId) -> Result<(), DieselError> {
    let uid = user_id;
    let fid = follow;
    {
        use crate::schema::followers::dsl::*;
        diesel::insert_into(followers)
            .values((user_id.eq(uid), follows.eq(fid)))
            .on_conflict((user_id, follows))
            .do_nothing()
            .execute(conn)
            .map(|_| ())
    }
}

pub fn unfollow(
    conn: &mut PgConnection,
    user_id: UserId,
    stop_following: UserId,
) -> Result<DeleteStatus, DieselError> {
    let uid = user_id;
    let fid = stop_following;
    {
        use crate::schema::followers::dsl::*;
        diesel::delete(followers)
            .filter(user_id.eq(uid))
            .filter(follows.eq(fid))
            .execute(conn)
            .map(|rowcount| {
                if rowcount > 0 {
                    DeleteStatus::Deleted
                } else {
                    DeleteStatus::NotFound
                }
            })
    }
}

pub fn is_following(
    conn: &mut PgConnection,
    user_id: UserId,
    is_following: UserId,
) -> Result<bool, DieselError> {
    let uid = user_id;
    let fid = is_following;
    {
        use crate::schema::followers::dsl::*;
        use diesel::dsl::count;

        followers
            .filter(user_id.eq(uid))
            .filter(follows.eq(fid))
            .select(count(user_id))
            .get_result(conn)
            .optional()
            .map(|n: Option<i64>| match n {
                Some(n) => n == 1,
                None => false,
            })
    }
}
