#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    access_token: String,
    refresh_token: Option<String>,
    user_id: String,
}

impl Session {
    pub fn new(access_token: String, refresh_token: Option<String>, user_id: String) -> Self {
        Self {
            access_token,
            refresh_token,
            user_id,
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }
}
