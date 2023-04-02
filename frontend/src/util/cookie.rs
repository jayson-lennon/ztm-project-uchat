#![allow(dead_code)]

use chrono::{DateTime, Utc};

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
