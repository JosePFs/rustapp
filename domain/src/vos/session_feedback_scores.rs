use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffortScore(i32);

impl EffortScore {
    pub fn new(value: i32) -> Result<Self> {
        if !(1..=10).contains(&value) {
            return Err(DomainError::InvalidParameter(
                "effort".to_string(),
                value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for EffortScore {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for EffortScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PainScore(i32);

impl PainScore {
    pub fn new(value: i32) -> Result<Self> {
        if !(0..=10).contains(&value) {
            return Err(DomainError::InvalidParameter(
                "pain".to_string(),
                value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for PainScore {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for PainScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effort_score_new() {
        let effort_score = EffortScore::new(5).unwrap();

        assert_eq!(effort_score.value(), 5);
    }

    #[test]
    fn effort_score_try_from_i32() {
        let effort_score = EffortScore::try_from(5).unwrap();

        assert_eq!(effort_score.value(), 5);
    }
}
