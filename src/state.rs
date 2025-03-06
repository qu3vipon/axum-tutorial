use axum::extract::FromRef;

use crate::model::TicketService;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub ticket_service: TicketService,
}
