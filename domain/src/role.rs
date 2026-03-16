use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Specialist,
    Patient,
}

impl Role {
    pub fn new(role: &str) -> Self {
        match role {
            "specialist" => Self::Specialist,
            "patient" => Self::Patient,
            _ => panic!("Invalid role: {}", role),
        }
    }
}

impl PartialEq<&Role> for Role {
    fn eq(&self, other: &&Role) -> bool {
        self.to_string() == other.to_string()
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
