use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::Error as JWTError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Auth errors
    LoginFail,
    AuthTokenNotProvided,
    AuthTokenEncodeFail { err: JWTError },
    AuthTokenDecodeFail { err: JWTError },
    AuthTokenExpired,

    // -- Model errors
    TicketNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANLDLED_ERROR").into_response()
    }
}
