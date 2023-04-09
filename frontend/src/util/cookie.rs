#![allow(dead_code)]

use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use uchat_domain::ids::SessionId;

use super::document;

pub fn get_session() -> Option<SessionId> {
    let cookies = document().cookie().unwrap();
    uchat_cookie::get_from_str(&cookies, "session_id").and_then(|id| SessionId::from_str(id).ok())
}

pub fn remove_session() {
    let cookie = format_cookie(
        format_kv(uchat_cookie::SESSION_ID, ""),
        Utc::now() - Duration::days(1),
    );
    document().set_cookie(&cookie).unwrap()
}

pub fn set_session(signature: String, id: SessionId, expires: DateTime<Utc>) {
    let cookie = format_cookie(format_kv(uchat_cookie::SESSION_ID, id.to_string()), expires);

    document().set_cookie(&cookie).unwrap();

    let cookie = format_cookie(
        format_kv(uchat_cookie::SESSION_SIGNATURE, signature),
        expires,
    );

    document().set_cookie(&cookie).unwrap();
}

#[cfg(not(debug_assertions))]
fn standard_options() -> &'static str {
    "SameSite=Strict; Path=/; Secure"
}

#[cfg(debug_assertions)]
fn standard_options() -> &'static str {
    "SameSite=Strict; Path=/;"
}

fn format_expiration(expires: DateTime<Utc>) -> String {
    expires.format("expires=%a, %d %b %Y %T GMT").to_string()
}

fn format_kv<K, V>(key: K, value: V) -> String
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    let key = key.as_ref();
    let value = value.as_ref();
    format!("{key}={value}")
}

fn format_cookie<S: AsRef<str>>(payload: S, expires: DateTime<Utc>) -> String {
    let expires = format_expiration(expires);
    let options = standard_options();
    let payload = payload.as_ref();

    let cookie = format!("{payload}; {expires}; {options}");
    cookie
}
