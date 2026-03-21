use std::fmt::{Display, Formatter};

use crate::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Specialist,
    Patient,
}

impl Role {
    pub fn value(&self) -> &str {
        match self {
            Role::Specialist => "specialist",
            Role::Patient => "patient",
        }
    }
}

impl PartialEq<str> for Role {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
    }
}

impl PartialEq<&str> for Role {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for Role {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::Specialist => "specialist",
                Role::Patient => "patient",
            }
        )
    }
}

impl TryFrom<&str> for Role {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "specialist" => Ok(Role::Specialist),
            "patient" => Ok(Role::Patient),
            _ => Err(DomainError::InvalidParameter(
                "role".to_string(),
                value.to_string(),
            )),
        }
    }
}

impl TryFrom<String> for Role {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_new() {
        let role = Role::try_from("specialist").unwrap();

        assert_eq!(role, Role::Specialist);
    }

    #[test]
    fn role_eq() {
        let role = Role::try_from("specialist").unwrap();
        let role2 = Role::try_from("specialist").unwrap();

        assert_eq!(role, role2);
    }

    #[test]
    fn role_eq_str() {
        let role = Role::try_from("specialist").unwrap();
        let str = "specialist";

        assert_eq!(role, str);
    }

    #[test]
    fn role_eq_string() {
        let role = Role::try_from("specialist").unwrap();
        let string = "specialist".to_string();

        assert_eq!(role, string);
    }

    #[test]
    fn role_try_from_str() {
        let role = Role::try_from("specialist").unwrap();

        assert_eq!(role, Role::Specialist);
    }

    #[test]
    fn role_try_from_string() {
        let role = Role::try_from("specialist".to_string()).unwrap();

        assert_eq!(role, Role::Specialist);
    }

    #[test]
    fn role_try_from_empty_string() {
        let result = Role::try_from("".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("role".to_string(), "".to_string())
        );
    }

    #[test]
    fn role_try_from_invalid_string() {
        let result = Role::try_from("invalid".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("role".to_string(), "invalid".to_string())
        );
    }
}
