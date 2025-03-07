use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::Error as JWTError;

use axum::Json;
use serde_json::json;
use uuid::Uuid;

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
        let (status_code, error) = self.status_code_and_message();
        let error_body = json!({
            "error": {
                "type": error.as_ref(),
                "request_uuid": Uuid::new_v4().to_string(),
            }
        });

        (status_code, Json(error_body)).into_response()
    }
}

impl Error {
    pub fn status_code_and_message(&self) -> (StatusCode, ClientError) {
        match self {
            // 401
            Self::LoginFail
            | Self::AuthTokenNotProvided
            | Self::AuthTokenDecodeFail { .. }
            | Self::AuthTokenExpired { .. } => {
                (StatusCode::UNAUTHORIZED, ClientError::UNAUTHORIZED)
            }
            // 404
            Self::TicketNotFound { .. } => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),
            // 500
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    BAD_REQUEST,
    UNAUTHORIZED,
    NOT_FOUND,
    SERVER_ERROR,
}
