#[derive(Clone, Debug)]
pub struct Context {
    user_id: u64,
}

impl Context {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
