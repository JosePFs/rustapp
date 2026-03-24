use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign},
};

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

impl Add<i32> for EffortScore {
    type Output = EffortScore;

    fn add(self, rhs: i32) -> EffortScore {
        EffortScore(self.0 + rhs)
    }
}

impl AddAssign for EffortScore {
    fn add_assign(&mut self, rhs: EffortScore) {
        self.0 += rhs.0;
    }
}

impl AddAssign<i32> for EffortScore {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
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

impl Add<i32> for PainScore {
    type Output = PainScore;

    fn add(self, rhs: i32) -> PainScore {
        PainScore(self.0 + rhs)
    }
}

impl AddAssign for PainScore {
    fn add_assign(&mut self, rhs: PainScore) {
        self.0 += rhs.0;
    }
}

impl AddAssign<i32> for PainScore {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
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
