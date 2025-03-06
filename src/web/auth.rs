use crate::web::{AUTH_SECRET, AUTH_TOKEN, AUTH_TOKEN_EXPIRY_HOURS};
use crate::{Error, Result};

use crate::context::Context;

use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

pub async fn cookie_authenticate(
    cookies: Cookies,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let _ = auth_token
        .ok_or(Error::AuthTokenNotProvided)
        .and_then(decode_access_token)?;

    Ok(next.run(request).await)
}

// JWT
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: u64,
    exp: i64,
}

pub fn encode_access_token(user_id: u64) -> Result<String> {
    let expiration = Utc::now() + Duration::hours(AUTH_TOKEN_EXPIRY_HOURS as i64);
    let claim = Claims {
        sub: user_id,
        exp: expiration.timestamp(),
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(AUTH_SECRET.as_ref()),
    )
    .map_err(|err| Error::AuthTokenEncodeFail { err })
}

pub fn decode_access_token(access_token: String) -> Result<u64> {
    let token_data = decode::<Claims>(
        &access_token,
        &DecodingKey::from_secret(AUTH_SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(|err| Error::AuthTokenDecodeFail { err })?;

    // check exp
    if token_data.claims.exp < Utc::now().timestamp() {
        return Err(Error::AuthTokenExpired);
    }

    Ok(token_data.claims.sub)
}

impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let cookies = Cookies::from_request_parts(parts, state).await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        let user_id = auth_token
            .ok_or(Error::AuthTokenNotProvided)
            .and_then(decode_access_token)?;

        Ok(Self::new(user_id))
    }
}
