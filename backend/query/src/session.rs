use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_domain::ids::{SessionId, UserId};

use crate::DieselError;

use crate::schema;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct Fingerprint(serde_json::Value);

impl From<serde_json::Value> for Fingerprint {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Queryable, Insertable)]
#[diesel(table_name = schema::web)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub fingerprint: Fingerprint,
}

pub fn new(
    conn: &mut PgConnection,
    user_id: UserId,
    duration: chrono::Duration,
    fingerprint: Fingerprint,
) -> Result<Session, DieselError> {
    let uid = user_id;
    let new_session = Session {
        id: SessionId::new(),
        user_id: uid,
        expires_at: Utc::now() + duration,
        created_at: Utc::now(),
        fingerprint,
    };
    {
        use crate::schema::web;
        diesel::insert_into(web::table)
            .values(&new_session)
            .on_conflict((web::user_id, web::fingerprint))
            .do_update()
            .set(web::expires_at.eq(new_session.expires_at))
            .get_result(conn)
    }
}

pub fn get(conn: &mut PgConnection, session_id: SessionId) -> Result<Option<Session>, DieselError> {
    use crate::schema::web;
    web::table
        .filter(web::id.eq(session_id))
        .get_result(conn)
        .optional()
}

pub fn find(
    conn: &mut PgConnection,
    user_id: UserId,
    fingerprint: Fingerprint,
) -> Result<Session, DieselError> {
    use crate::schema::web;
    web::table
        .filter(web::user_id.eq(user_id))
        .filter(web::fingerprint.eq(fingerprint))
        .get_result(conn)
}
