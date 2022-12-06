use std::str::FromStr;

use anyhow::{anyhow, bail};

use super::Day;

pub const DAY_FOUR: Day = Day { part_one, part_two };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct RangeInclusive {
    min: i32,
    max: i32,
}

impl PartialOrd for RangeInclusive {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeInclusive {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min.cmp(&other.min)
    }
}

impl FromStr for RangeInclusive {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let min: i32 = parts
            .next()
            .ok_or_else(|| anyhow!("Invalid range: {}", s))?
            .parse()?;
        let max: i32 = parts
            .next()
            .ok_or_else(|| anyhow!("Invalid range: {}", s))?
            .parse()?;

        if parts.next().is_some() {
            bail!("Invalid range: {}", s);
        }

        Ok(RangeInclusive { min, max })
    }
}

impl RangeInclusive {
    fn contains(&self, other: &RangeInclusive) -> bool {
        self.min <= other.min && self.max >= other.max
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct AssignmentPair {
    left: RangeInclusive,
    right: RangeInclusive,
}

impl FromStr for AssignmentPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let left: RangeInclusive = parts
            .next()
            .ok_or_else(|| anyhow!("Invalid assignment pair: {}", s))?
            .parse()?;
        let right: RangeInclusive = parts
            .next()
            .ok_or_else(|| anyhow!("Invalid assignment pair: {}", s))?
            .parse()?;

        if parts.next().is_some() {
            bail!("Invalid assignment pair: {}", s);
        }

        Ok(AssignmentPair { left, right })
    }
}

impl AssignmentPair {
    fn one_contains_other(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn overlaps(&self) -> bool {
        let (first, second) = if self.left.min < self.right.min {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        first.max >= second.min
    }
}

pub fn part_one(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<AssignmentPair>())
        .map(|result| result.unwrap())
        .filter(|pair| pair.one_contains_other())
        .count()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<AssignmentPair>())
        .map(|result| result.unwrap())
        .filter(|pair| pair.overlaps())
        .count()
        .to_string()
}
