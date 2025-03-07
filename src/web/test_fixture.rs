use tower_cookies::Cookie;

use crate::web::auth::jwt::encode_access_token;
use crate::web::AUTH_TOKEN;

pub struct CommonFixture<'a> {
    cookie: Cookie<'a>,
}

impl<'a> CommonFixture<'a> {
    pub fn new() -> Self {
        let access_token = encode_access_token(1_u64).unwrap();

        Self {
            cookie: Cookie::new(AUTH_TOKEN, access_token),
        }
    }

    pub fn cookie(&self) -> &Cookie {
        &self.cookie
    }
}
