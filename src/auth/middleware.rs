use crate::{Error, Result};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};

use crate::auth::extractor::AuthContext;
use crate::auth::jwt::decode_access_token;
use crate::auth::AUTH_TOKEN;

pub async fn auth_context_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let auth_context = match auth_token
        .ok_or(Error::AuthTokenNotProvided)
        .and_then(decode_access_token)
    {
        Ok(user_id) => Ok(AuthContext::new(user_id)),
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than AuthTokenNotProvided.
    if auth_context.is_err() && !matches!(auth_context, Err(Error::AuthTokenNotProvided)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the auth_context in the request extension.
    req.extensions_mut().insert(auth_context?);

    Ok(next.run(req).await)
}

pub async fn cookie_authenticate(
    auth_context: Result<AuthContext>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    auth_context?;
    Ok(next.run(request).await)
}
