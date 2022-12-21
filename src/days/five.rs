use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Context, Ok};
use itertools::Itertools;

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
    pub from: usize,
    pub to: usize,
    pub count: usize,
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

impl CraneYard {
    pub fn handle_instruction(&mut self, instruction: &Instruction) {
        let to = instruction.to - 1;
        let from = instruction.from - 1;
        let buffer = {
            let mut buffer = Vec::new();
            let from_stack = self.crates.get_mut(&from).unwrap();
            for _ in 0..instruction.count {
                buffer.push(from_stack.pop().unwrap());
            }
            buffer
        };
        let to_stack = self.crates.get_mut(&to).unwrap();
        to_stack.extend(buffer.into_iter());
    }

    pub fn handle_instruction_two(&mut self, instruction: &Instruction) {
        let to = instruction.to - 1;
        let from = instruction.from - 1;
        let buffer = {
            let mut buffer = Vec::new();
            let from_stack = self.crates.get_mut(&from).unwrap();
            for _ in 0..instruction.count {
                buffer.push(from_stack.pop().unwrap());
            }
            buffer.reverse();
            buffer
        };
        let to_stack = self.crates.get_mut(&to).unwrap();
        to_stack.extend(buffer.into_iter());
    }

    pub fn get_secret_message(&self) -> String {
        let size = self.crates.keys().max().unwrap() + 1;
        let mut message = String::new();
        for i in 0..size {
            let stack = self.crates.get(&i).unwrap();
            let char = stack.last().unwrap_or(&' ');
            message.push(*char);
        }
        message
    }
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

        if !idx_row
            .iter()
            .all(|t| matches!(t, DiagramToken::CrateIdx(_)))
        {
            bail!("Invalid index row")
        }

        let mut crates: HashMap<usize, Vec<char>> = HashMap::new();

        for line in lines {
            for (idx, token) in line
                .chars()
                .chunks(4)
                .into_iter()
                .map(|chunk| {
                    chunk
                        .collect::<String>()
                        .parse::<DiagramToken>()
                        .context("Invalid token")
                        .unwrap()
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

    let mut yard = yard.parse::<CraneYard>().unwrap();

    let commands = commands
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    for command in commands.iter() {
        yard.handle_instruction(command);
    }

    yard.get_secret_message()
}

fn part_two(input: &str) -> String {
    let (yard, commands) = input.split_once("\n\n").unwrap();

    let mut yard = yard.parse::<CraneYard>().unwrap();

    let commands = commands
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    for command in commands.iter() {
        yard.handle_instruction_two(command);
    }

    yard.get_secret_message()
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_part_one_example() {
        let example = include_str!("../../inputs/5/1/example.txt");
        let answer = part_one(example);
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn test_part_one_input() {
        let example = include_str!("../../inputs/5/1/input.txt");
        let answer = part_one(example);
        assert_eq!(answer, "RFFFWBPNS");
    }

    #[test]
    fn test_part_two_example() {
        let example = include_str!("../../inputs/5/2/example.txt");
        let answer = part_two(example);
        assert_eq!(answer, "MCD");
    }

    #[test]
    fn test_part_two_input() {
        let example = include_str!("../../inputs/5/2/input.txt");
        let answer = part_two(example);
        assert_eq!(answer, "CQQBBJFCS");
    }
}
