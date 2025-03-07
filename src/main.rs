use axum::{middleware, Router};
use model::TicketService;
use state::AppState;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::error::{Error, Result};

mod error;
mod extractor;
mod model;
mod state;
mod web;

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
        .nest("/api", web::routes_login::routes())
        .nest(
            "/api",
            web::routes_tickets::routes(state)
                .route_layer(middleware::from_fn(web::auth::cookie_authenticate)),
        )
        .layer(CookieManagerLayer::new())
}
