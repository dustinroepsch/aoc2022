use anyhow::anyhow;
use itertools::Itertools;
use std::{
    fmt::{Display, Pointer},
    str::FromStr,
};

use super::Day;

pub const DAY_SEVEN: Day = Day { part_one, part_two };

#[derive(Debug)]
enum Token {
    CD(String),
    LS,
    Dir(String),
    File { size: usize, name: String },
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::CD(dir) => write!(f, "cd ({})", dir),
            Token::LS => write!(f, "ls"),
            Token::Dir(dir) => write!(f, "dir ({})", dir),
            Token::File { size, name } => write!(f, "file, size ({}) name ({})", size, name),
        }
    }
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let first_word = words.next();
        let second_word = words.next();
        let third_word = words.next();

        match (first_word, second_word, third_word) {
            (Some("$"), Some("ls"), None) => Ok(Token::LS),
            (Some("$"), Some("dir"), Some(dir)) => Ok(Token::Dir(dir.to_string())),
            (Some(size), Some(name), None) => Ok(Token::File {
                size: size.parse()?,

                name: name.to_string(),
            }),
            (_, _, _) => Err(anyhow!("{} is not a valid token", s)),
        }
    }
}

fn part_one(input: &str) -> String {
    input
        .lines()
        .map(|s| Token::from_str(s).unwrap())
        .join("\n")
}

fn part_two(input: &str) -> String {
    todo!()
}
