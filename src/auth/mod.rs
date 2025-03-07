pub mod extractor;
pub mod jwt;
pub mod middleware;

pub const AUTH_TOKEN: &str = "access-token";
pub const AUTH_SECRET: &str = "super-secure";
pub const AUTH_TOKEN_EXPIRY_HOURS: u8 = 24 * 7;
