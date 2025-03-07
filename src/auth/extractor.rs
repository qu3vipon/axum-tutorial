use crate::{Error, Result};

use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Clone, Debug)]
pub struct AuthContext {
    user_id: u64,
}

impl AuthContext {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

impl<S: Send + Sync> FromRequestParts<S> for AuthContext {
    type Rejection = Error;

    // this can be called multiple times
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let auth_context = parts
            .extensions
            .get::<AuthContext>()
            .ok_or(Error::AuthTokenNotProvided)?
            .clone();

        Ok(auth_context)
    }
}
