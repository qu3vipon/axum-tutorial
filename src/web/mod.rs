pub mod auth;
pub mod routes_login;
pub mod routes_tickets;

pub const AUTH_TOKEN: &str = "access-token";
pub const AUTH_SECRET: &str = "super-secure";
pub const AUTH_TOKEN_EXPIRY_HOURS: u8 = 24 * 7;
