use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::bail;

use super::Day;

pub const DAY_THREE: Day = Day {
    part_one: part_one,
    part_two: part_two,
};

type Compartment = HashMap<char, usize>;

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

        let a = a.chars().fold(Compartment::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let b = b.chars().fold(Compartment::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        Ok(RuckSack { a, b })
    }
}

impl RuckSack {
    fn overlap(&self) -> char {
        let a = self.a.keys().cloned().collect::<HashSet<char>>();
        let b = self.b.keys().cloned().collect::<HashSet<char>>();
        a.intersection(&b).next().unwrap().clone()
    }
}

pub fn part_one(input: &str) -> String {
    input
        .lines()
        .map(RuckSack::from_str)
        .map(Result::unwrap)
        .map(|r| r.overlap())
        .map(|c| c.priority())
        .sum::<i32>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    todo!()
}
