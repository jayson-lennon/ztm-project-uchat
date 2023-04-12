pub mod home;
pub mod login;
pub mod new_post;
pub mod register;
pub mod trending;

pub use home::Home;
pub use login::Login;
pub use new_post::*;
pub use register::Register;
pub use trending::Trending;

pub use route::*;

pub mod route {
    pub const ACCOUNT_LOGIN: &str = "/account/login";
    pub const ACCOUNT_REGISTER: &str = "/account/register";
    pub const HOME: &str = "/home";
    pub const POST_NEW_CHAT: &str = "/post/new_chat";
    pub const POSTS_TRENDING: &str = "/posts/trending";
}
