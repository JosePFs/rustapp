use chrono::{DateTime, Utc};

use domain::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Credentials {
    email: Email,
    password: Password,
}

impl Credentials {
    pub fn new(email: Email, password: Password) -> Self {
        Self { email, password }
    }

    pub fn from(email: &str, password: &str) -> Self {
        Self::new(
            Email::new(email.to_string()),
            Password::new(password.to_string()),
        )
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    access_token: String,
    refresh_token: Option<String>,
    user_id: String,
    expires_at: Option<DateTime<Utc>>,
}

impl Session {
    pub fn new(
        access_token: String,
        refresh_token: Option<String>,
        user_id: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            user_id,
            expires_at,
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

    pub fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }

    pub fn should_refresh(&self) -> bool {
        match self.expires_at {
            Some(expires) => {
                let five_minutes = chrono::Duration::minutes(5);
                expires - five_minutes < Utc::now()
            }
            None => false,
        }
    }
}

#[common::async_trait_platform]
pub trait AuthService: Send + Sync {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session>;
    async fn refresh_session(&self, refresh_token: &str) -> Result<Session>;
    fn get_session(&self) -> Option<Session>;
}
