use crate::error::{Error, Result};

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::web;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/login", post(login_handler))
}

async fn login_handler(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "admin" || payload.password != "1234" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": "ok"
        }
    }));

    Ok(body)
}

#[cfg(test)]
mod tests {

    use crate::app;
    use crate::model::TicketService;
    use crate::state::AppState;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test]
    async fn login() {
        let app = app(AppState {
            ticket_service: TicketService::new().unwrap(),
        });
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/login")
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "username": "admin",
                            "password": "1234"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "result": {"success": "ok"} }));
    }
}
