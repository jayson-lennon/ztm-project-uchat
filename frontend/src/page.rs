pub mod login;
pub mod register;

pub use login::Login;
pub use register::Register;

pub use route::*;

pub mod route {
    pub const ACCOUNT_LOGIN: &str = "/account/login";
    pub const ACCOUNT_REGISTER: &str = "/account/register";
}
