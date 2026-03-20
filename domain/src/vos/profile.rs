use std::fmt::{Display, Formatter};

use crate::vos::{email::Email, fullname::FullName, id::Id, role::Role};

#[derive(Debug, Clone, Eq, Hash)]
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

impl PartialEq<Profile> for Profile {
    fn eq(&self, other: &Profile) -> bool {
        self.id == other.id
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Profile(id: {}, email: {}, full_name: {}, role: {})",
            self.id.value(),
            self.email.value(),
            self.full_name.value(),
            self.role.value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_new() {
        let profile = default_profile(None);

        assert_eq!(profile.id(), &default_id());
        assert_eq!(profile.email(), &default_email());
        assert_eq!(profile.full_name(), &default_full_name());
        assert_eq!(profile.role(), &default_role());
    }

    #[test]
    fn test_profile_eq() {
        let profile_one = default_profile(None);
        let profile_two = default_profile(None);

        assert_eq!(profile_one, profile_two);
    }

    #[test]
    fn test_profile_not_eq() {
        let profile_one = default_profile(None);
        let profile_two = default_profile(Some(
            Id::try_from("123e4567-e89b-12d3-a456-426614174001").unwrap(),
        ));

        assert_ne!(profile_one, profile_two);
    }

    #[test]
    fn test_profile_to_string() {
        let profile = default_profile(None);

        assert_eq!(profile.to_string(), "Profile(id: 123e4567-e89b-12d3-a456-426614174000, email: test@example.com, full_name: John Doe, role: specialist)");
    }

    fn default_id() -> Id {
        Id::try_from("123e4567-e89b-12d3-a456-426614174000").unwrap()
    }

    fn default_email() -> Email {
        Email::try_from("test@example.com").unwrap()
    }

    fn default_full_name() -> FullName {
        FullName::try_from("John Doe").unwrap()
    }

    fn default_role() -> Role {
        Role::Specialist
    }

    fn default_profile(id: Option<Id>) -> Profile {
        Profile::new(
            id.unwrap_or(default_id()),
            default_email(),
            default_full_name(),
            default_role(),
        )
    }
}
