use crate::vos::{email::Email, fullname::FullName, role::Role};

#[derive(Debug, Clone)]
pub struct User {
    id: String,
    fullname: FullName,
    email: Email,
    role: Role,
}

impl User {
    pub fn new(id: String, fullname: FullName, email: Email, role: Role) -> Self {
        Self {
            id,
            fullname,
            email,
            role,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn fullname(&self) -> &FullName {
        &self.fullname
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn role(&self) -> &Role {
        &self.role
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
