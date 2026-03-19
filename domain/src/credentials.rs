use crate::email::Email;

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
