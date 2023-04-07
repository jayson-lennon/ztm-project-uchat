use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub struct ApiError {
    pub code: Option<StatusCode>,
    pub err: color_eyre::Report,
}

pub fn err_response<T: Into<String>>(code: StatusCode, msg: T) -> Response {
    (
        code,
        Json(uchat_endpoint::RequestFailed { msg: msg.into() }),
    )
        .into_response()
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if let Some(code) = self.code {
            return err_response(code, format!("{}", self.err));
        }

        err_response(StatusCode::INTERNAL_SERVER_ERROR, "server error")
    }
}

impl<E> From<E> for ApiError
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self {
            code: None,
            err: err.into(),
        }
    }
}
