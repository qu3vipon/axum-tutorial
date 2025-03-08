use axum::{extract::MatchedPath, http::Request, middleware, response::Response, Router};
use model::TicketService;
use state::AppState;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::error::{Error, Result};
use std::time::Duration;

mod auth;
mod error;
mod model;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app(AppState {
            ticket_service: TicketService::new().unwrap(),
        }),
    )
    .await
    .unwrap();

    Ok(())
}

fn app(state: AppState) -> Router {
    Router::new()
        .nest("/api", routes::login::routes())
        .nest(
            "/api",
            routes::tickets::routes(state)
                .route_layer(middleware::from_fn(auth::middleware::cookie_authenticate))
                .route_layer(middleware::from_fn(auth::middleware::auth_context_resolver)),
        )
        .layer(CookieManagerLayer::new())
        .layer(middleware::map_response(response_mapper))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::debug!("{:?}", request);
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    tracing::debug!("Response {}, latency: {:?}", _response.status(), _latency);
                }),
        )
}

async fn response_mapper(res: Response) -> Response {
    // TODO: log origin error & return client error
    res
}
