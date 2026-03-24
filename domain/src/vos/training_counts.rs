use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DayIndex(i32);

impl DayIndex {
    pub const ZERO: Self = Self(0);

    pub fn new(value: i32) -> Result<Self> {
        if value < 0 {
            return Err(DomainError::InvalidParameter(
                "day_index".to_string(),
                value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for DayIndex {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for DayIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<i32> for DayIndex {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleOrderIndex(i32);

impl ScheduleOrderIndex {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);

    pub fn new(value: i32) -> Result<Self> {
        if value < 0 {
            return Err(DomainError::InvalidParameter(
                "order_index".to_string(),
                value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for ScheduleOrderIndex {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for ScheduleOrderIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DaysInBlock(i32);

impl DaysInBlock {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);

    pub fn new(value: i32) -> Result<Self> {
        if value < 1 {
            return Err(DomainError::InvalidParameter(
                "days_count".to_string(),
                value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for DaysInBlock {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sets(i32);

impl Sets {
    pub fn new(value: i32) -> Result<Self> {
        if value < 1 {
            return Err(DomainError::InvalidParameter(
                "sets".to_string(),
                value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for Sets {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reps(i32);

impl Reps {
    pub fn new(value: i32) -> Result<Self> {
        if value < 1 {
            return Err(DomainError::InvalidParameter(
                "reps".to_string(),
                value.to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for Reps {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_index_new() {
        let day_index = DayIndex::new(5).unwrap();

        assert_eq!(day_index.value(), 5);
    }

    #[test]
    fn day_index_try_from_i32() {
        let day_index = DayIndex::try_from(5).unwrap();

        assert_eq!(day_index.value(), 5);
    }

    #[test]
    fn schedule_order_index_new() {
        let schedule_order_index = ScheduleOrderIndex::new(5).unwrap();

        assert_eq!(schedule_order_index.value(), 5);
    }

    #[test]
    fn schedule_order_index_try_from_i32() {
        let schedule_order_index = ScheduleOrderIndex::try_from(5).unwrap();

        assert_eq!(schedule_order_index.value(), 5);
    }

    #[test]
    fn days_in_block_new() {
        let days_in_block = DaysInBlock::new(5).unwrap();

        assert_eq!(days_in_block.value(), 5);
    }

    #[test]
    fn days_in_block_try_from_i32() {
        let days_in_block = DaysInBlock::try_from(5).unwrap();

        assert_eq!(days_in_block.value(), 5);
    }

    #[test]
    fn sets_new() {
        let sets = Sets::new(5).unwrap();

        assert_eq!(sets.value(), 5);
    }

    #[test]
    fn sets_try_from_i32() {
        let sets = Sets::try_from(5).unwrap();

        assert_eq!(sets.value(), 5);
    }

    #[test]
    fn reps_new() {
        let reps = Reps::new(5).unwrap();

        assert_eq!(reps.value(), 5);
    }

    #[test]
    fn reps_try_from_i32() {
        let reps = Reps::try_from(5).unwrap();

        assert_eq!(reps.value(), 5);
    }
}
