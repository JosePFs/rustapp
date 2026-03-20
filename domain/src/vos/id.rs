use std::fmt::{Display, Formatter};

use uuid::Uuid;

use crate::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.0.to_string() == other
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for Id {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Id {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value)
            .map(Self::new)
            .map_err(|_| DomainError::InvalidParameter("id".to_string(), value.to_string()))
    }
}

impl TryFrom<String> for Id {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_new() {
        let id = Id::new(Uuid::new_v4());

        assert_eq!(id.value(), id.value());
    }

    #[test]
    fn test_id_to_string() {
        let id = Id::new(Uuid::new_v4());
        let string = id.to_string();

        assert_eq!(string, id.value().to_string());
    }

    #[test]
    fn test_id_eq() {
        let uuid = Uuid::new_v4();
        let uuid_one = uuid.clone();
        let uuid_two = uuid.clone();

        let id = Id::new(uuid_one);
        let id2 = Id::new(uuid_two);

        assert_eq!(id, id2);
    }

    #[test]
    fn test_id_eq_str() {
        let id = Id::new(Uuid::new_v4());
        let str = id.value().to_string();

        assert_eq!(id, str.as_str());
    }

    #[test]
    fn test_id_eq_string() {
        let id = Id::new(Uuid::new_v4());
        let string = id.value().to_string();

        assert_eq!(id, string);
    }

    #[test]
    fn test_id_try_from_str() {
        let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
        let id = Id::try_from(uuid_str).unwrap();

        assert_eq!(id.value(), Uuid::parse_str(uuid_str).unwrap());
    }

    #[test]
    fn test_id_try_from_string() {
        let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
        let id = Id::try_from(uuid_str.to_string()).unwrap();

        assert_eq!(id.value(), Uuid::parse_str(uuid_str).unwrap());
    }

    #[test]
    fn test_id_try_from_empty_string() {
        let result = Id::try_from("".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("id".to_string(), "".to_string())
        );
    }

    #[test]
    fn test_id_try_from_invalid_string() {
        let result = Id::try_from("invalid".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("id".to_string(), "invalid".to_string())
        );
    }
}
