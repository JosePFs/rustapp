#[derive(Clone, Debug)]
pub struct Session {
    access_token: String,
    user_id: String,
}

impl Session {
    pub fn new(access_token: String, user_id: String) -> Self {
        Self {
            access_token,
            user_id,
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}
