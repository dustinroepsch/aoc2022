use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

use super::Day;

pub const DAY_EIGHT: Day = Day { part_one, part_two };

struct TreeHeightMap {
    height: Vec<Vec<u8>>,
}

impl Display for TreeHeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.height {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl FromStr for TreeHeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height: Vec<Vec<u8>> = Vec::new();

        for line in s.lines() {
            let mut row: Vec<u8> = Vec::new();

            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }

            height.push(row);
        }

        Ok(TreeHeightMap { height })
    }
}

fn part_one(input: &str) -> String {
    let m: TreeHeightMap = input.parse().unwrap();
    format!("{}", m)
}

fn part_two(input: &str) -> String {
    todo!()
}
