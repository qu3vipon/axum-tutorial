use crate::model::{Ticket, TicketRequest, TicketService};
use crate::state::AppState;
use crate::Result;

use axum::extract::{Path, State};
use axum::{Json, Router};

use axum::routing::{delete, post};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/tickets",
            post(create_ticket_handler).get(list_tickets_handler),
        )
        .route("/tickets/{id}", delete(delete_ticket_handler))
        .with_state(state)
}

async fn create_ticket_handler(
    State(service): State<TicketService>,
    Json(ticket_request): Json<TicketRequest>,
) -> Result<Json<Ticket>> {
    let ticket = service.create_ticket(ticket_request).await?;
    Ok(Json(ticket))
}

async fn list_tickets_handler(State(service): State<TicketService>) -> Result<Json<Vec<Ticket>>> {
    let tickets = service.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket_handler(
    State(service): State<TicketService>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    let ticket = service.delete_ticket(id).await?;
    Ok(Json(ticket))
}

#[cfg(test)]
mod tests {
    use crate::app;
    use crate::model::TicketRequest;
    use crate::model::TicketService;
    use crate::state::AppState;
    
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;
    

    use crate::web::test_fixture::CommonFixture;

    #[tokio::test]
    async fn create_ticket() {
        let app = app(AppState {
            ticket_service: TicketService::new().unwrap(),
        });

        let fixture = CommonFixture::new();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/tickets")
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("Cookie", fixture.cookie().to_string())
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "title": "t"
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
        assert_eq!(body, json!({ "id": 0, "title": "t" }));
    }

    #[tokio::test]
    async fn list_tickets() {
        let fixture = CommonFixture::new();

        let app_state = AppState {
            ticket_service: TicketService::new().unwrap(),
        };

        app_state
            .ticket_service
            .create_ticket(TicketRequest {
                title: "t".to_string(),
            })
            .await
            .unwrap();

        let app = app(app_state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/tickets")
                    .header("Cookie", fixture.cookie().to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!([{ "id": 0, "title": "t" }]));
    }

    #[tokio::test]
    async fn delete_ticket() {
        let fixture = CommonFixture::new();

        let app_state = AppState {
            ticket_service: TicketService::new().unwrap(),
        };

        app_state
            .ticket_service
            .create_ticket(TicketRequest {
                title: "t".to_string(),
            })
            .await
            .unwrap();

        let app = app(app_state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/tickets/0")
                    .method(http::Method::DELETE)
                    .header("Cookie", fixture.cookie().to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "id": 0, "title": "t" }));
    }
}
