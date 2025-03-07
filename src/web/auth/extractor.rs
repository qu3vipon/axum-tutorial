use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use tower_cookies::Cookies;

use crate::web::auth::jwt::decode_access_token;

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

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        // get auth context from cookies
        let cookies = Cookies::from_request_parts(parts, state).await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        let user_id = auth_token
            .ok_or(Error::AuthTokenNotProvided)
            .and_then(decode_access_token)?;

        Ok(Self::new(user_id))
    }
}
