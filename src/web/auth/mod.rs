pub mod extractor;
pub mod jwt;

use crate::Result;

use crate::web::auth::extractor::AuthContext;

use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;


// auth
pub async fn cookie_authenticate(
    auth_context: Result<AuthContext>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    auth_context?;
    Ok(next.run(request).await)
}
