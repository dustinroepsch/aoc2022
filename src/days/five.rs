use std::{
    collections::HashMap,
    hash::Hash,
    str::{Chars, FromStr},
};

use anyhow::{bail, Context, Ok};
use itertools::{Chunk, Itertools};

use super::Day;

pub const DAY_FIVE: Day = Day { part_one, part_two };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagramToken {
    Empty,
    Crate(char),
    CrateIdx(usize),
}

impl FromStr for DiagramToken {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let first = chars.next().context("Not enough chars to be a token")?;
        let second = chars.next().context("Not enough chars to be a token")?;
        let third = chars.next().context("Not enough chars to be a token")?;

        let fourth = chars.next();
        if let Some(c) = fourth {
            if !c.is_whitespace() {
                return Err(anyhow::anyhow!("Too many chars to be a token"));
            }
        }

        match (first, second, third) {
            ('[', c, ']') => Ok(DiagramToken::Crate(c)),
            (' ', ' ', ' ') => Ok(DiagramToken::Empty),
            (' ', d, ' ') => Ok(DiagramToken::CrateIdx(
                d.to_digit(10).context("Not a digit")? as usize,
            )),
            _ => Err(anyhow::anyhow!("Invalid token")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        if "move" != words.next().context("Not enough words")? {
            bail!("Expected 'move'");
        }

        let count = words
            .next()
            .context("Not enough words")?
            .parse::<usize>()
            .context("Not a number")?;

        if "from" != words.next().context("Not enough words")? {
            bail!("Expected 'from'");
        }

        let from = words
            .next()
            .context("Not enough words")?
            .parse::<usize>()
            .context("Not a number")?;

        if "to" != words.next().context("Not enough words")? {
            bail!("Expected 'to'");
        }

        let to = words
            .next()
            .context("Not enough words")?
            .parse::<usize>()
            .context("Not a number")?;

        Ok(Self { from, to, count })
    }
}

#[derive(Debug)]
struct CraneYard {
    crates: HashMap<usize, Vec<char>>,
}

impl FromStr for CraneYard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        let idx_row = lines.next().context("not enough rows")?;
        let idx_row = idx_row
            .chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .map(|s| s.parse::<DiagramToken>())
            .collect::<Result<Vec<_>, _>>()?;

        if !idx_row.iter().all(|t| match t {
            DiagramToken::CrateIdx(_) => true,
            _ => false,
        }) {
            bail!("Invalid index row");
        }

        let mut crates: HashMap<usize, Vec<char>> = HashMap::new();

        while let Some(line) = lines.next() {
            for (idx, token) in line
                .chars()
                .chunks(4)
                .into_iter()
                .map(|chunk| {
                    chunk
                        .collect::<String>()
                        .parse::<DiagramToken>()
                        .context("Invalid token").unwrap()
                })
                .enumerate()
            {
                match token {
                    DiagramToken::Crate(c) => {
                        crates.entry(idx).or_default().push(c);
                    }
                    DiagramToken::Empty => {}
                    DiagramToken::CrateIdx(_) => {}
                }
            }
        }

        Ok(Self { crates })
    }
}

fn part_one(input: &str) -> String {
    let (yard, commands) = input.split_once("\n\n").unwrap();

    let yard = yard.parse::<CraneYard>().unwrap();

    let commands = commands
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    format!("({:?}) - ({:?})", yard, commands)
}

fn part_two(_input: &str) -> String {
    todo!()
}
