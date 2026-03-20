use crate::vos::{email::Email, fullname::FullName, id::Id, role::Role};

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    id: Id,
    email: Email,
    full_name: FullName,
    role: Role,
}

impl Profile {
    pub fn new(id: Id, email: Email, full_name: FullName, role: Role) -> Self {
        Self {
            id,
            email,
            full_name,
            role,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    pub fn role(&self) -> &Role {
        &self.role
    }
}
