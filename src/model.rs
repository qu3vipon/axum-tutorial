use crate::{web::auth::extractor::AuthContext, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketRequest {
    pub title: String,
}

#[derive(Clone)]
pub struct TicketService {
    ticket_repo: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl TicketService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ticket_repo: Arc::default(),
        })
    }
}

impl TicketService {
    pub async fn create_ticket(
        &self,
        auth_context: AuthContext,
        ticket_request: TicketRequest,
    ) -> Result<Ticket> {
        let mut store = self.ticket_repo.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            user_id: auth_context.user_id(),
            title: ticket_request.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, auth_context: AuthContext) -> Result<Vec<Ticket>> {
        let store = self.ticket_repo.lock().unwrap();
        let tickets = store
            .iter()
            .filter_map(|t| {
                if let Some(ticket) = t {
                    if ticket.user_id == auth_context.user_id() {
                        Some(ticket.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, auth_context: AuthContext, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_repo.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| {
            if let Some(ticket) = t {
                if ticket.user_id == auth_context.user_id() {
                    t.take()
                } else {
                    None
                }
            } else {
                None
            }
        });

        ticket.ok_or(Error::TicketNotFound { id })
    }
}
