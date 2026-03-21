use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

const MAX_LABEL_LEN: usize = 256;

fn validate_label(raw: &str, param: &str) -> Result<String> {
    let s = raw.trim();
    if s.is_empty() {
        return Err(DomainError::InvalidParameter(
            param.to_string(),
            "(empty)".to_string(),
        ));
    }
    if s.len() > MAX_LABEL_LEN {
        return Err(DomainError::InvalidParameter(
            param.to_string(),
            format!("len {}", s.len()),
        ));
    }
    Ok(s.to_string())
}

macro_rules! label_vo {
    ($name:ident, $param:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn value(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<&str> for $name {
            type Error = DomainError;

            fn try_from(value: &str) -> Result<Self> {
                validate_label(value, $param).map($name)
            }
        }

        impl TryFrom<String> for $name {
            type Error = DomainError;

            fn try_from(value: String) -> Result<Self> {
                Self::try_from(value.as_str())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

label_vo!(ProgramName, "program_name");
label_vo!(WorkoutName, "workout_name");
label_vo!(ExerciseName, "exercise_name");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_name_non_empty() {
        assert!(ProgramName::try_from("").is_err());
        assert_eq!(ProgramName::try_from("  A  ").unwrap().value(), "A");
    }

    #[test]
    fn program_name_from_string() {
        let program_name = ProgramName::try_from("A".to_string()).unwrap();

        assert_eq!(program_name.value(), "A");
    }

    #[test]
    fn max_length() {
        let result = ProgramName::try_from("a".repeat(MAX_LABEL_LEN + 1));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter(
                "program_name".to_string(),
                format!("len {}", MAX_LABEL_LEN + 1)
            )
        );
    }
}
