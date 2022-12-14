use std::{collections::HashSet, str::FromStr};

use anyhow::bail;
use itertools::Itertools;

use super::Day;

pub const DAY_THREE: Day = Day { part_one, part_two };

type Compartment = HashSet<char>;

struct RuckSack {
    a: Compartment,
    b: Compartment,
}

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for char {
    fn priority(&self) -> i32 {
        if !self.is_alphabetic() {
            panic!("Not alphabetic");
        }

        if self.is_lowercase() {
            (*self as i32) - ('a' as i32) + 1
        } else {
            (*self as i32) - ('A' as i32) + 27
        }
    }
}

impl FromStr for RuckSack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            bail!("Invalid Rucksack (not even) {}", s);
        }

        let (a, b) = s.split_at(s.len() / 2);

        Ok(RuckSack {
            a: a.chars().collect(),
            b: b.chars().collect(),
        })
    }
}

impl RuckSack {
    fn overlap(&self) -> HashSet<char> {
        self.a.intersection(&self.b).cloned().collect()
    }
}

pub fn part_one(input: &str) -> String {
    input
        .lines()
        .map(RuckSack::from_str)
        .map(Result::unwrap)
        .flat_map(|r| r.overlap())
        .map(|c| c.priority())
        .sum::<i32>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| chunk.reduce(|a, b| a.intersection(&b).cloned().collect()))
        .flatten()
        .map(|c| c.priority())
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_part_one_example() {
        let example = include_str!("../../inputs/3/1/example.txt");
        let answer = part_one(example);
        assert_eq!(answer, "157");
    }

    #[test]
    fn test_part_one_input() {
        let example = include_str!("../../inputs/3/1/input.txt");
        let answer = part_one(example);
        assert_eq!(answer, "7581");
    }

    #[test]
    fn test_part_two_example() {
        let example = include_str!("../../inputs/3/2/example.txt");
        let answer = part_two(example);
        assert_eq!(answer, "70");
    }

    #[test]
    fn test_part_two_input() {
        let example = include_str!("../../inputs/3/2/input.txt");
        let answer = part_two(example);
        assert_eq!(answer, "2525");
    }
}
